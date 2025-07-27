use std::sync::Arc;

use n0_future::StreamExt;
use ps_datachunk::{DataChunk, OwnedDataChunk};
use ps_hash::Hash;
use ps_hkey::{AsyncStore, PsHkeyError};
use ps_promise::{Promise, PromiseRejection};

use crate::{Interaction, InteractionReadPacketError, InteractionSendPacketError, Packet};

impl AsyncStore for Interaction {
    type Chunk = OwnedDataChunk;
    type Error = InteractionAsyncStoreError;

    fn get(&self, hash: &Hash) -> Promise<Self::Chunk, Self::Error> {
        let hash = hash.to_string();
        let mut interaction = self.clone();

        Promise::new(async move {
            interaction
                .send_packet(crate::Packet::FetchRequest(crate::FetchRequest {
                    hash: hash.to_string(),
                }))
                .await?;

            let packet = match interaction.next().await {
                None => Err(InteractionAsyncStoreError::NoResponse)?,
                Some(Ok(packet)) => packet,
                Some(Err(err)) => Err(err)?,
            };

            if let Packet::FetchResponse(response) = packet {
                match response {
                    crate::FetchResponse::Error(err) => {
                        Err(InteractionAsyncStoreError::ReturnedError(err))?
                    }
                    crate::FetchResponse::NotFound => Err(InteractionAsyncStoreError::NotFound)?,
                    crate::FetchResponse::Success(bytes) => Ok(OwnedDataChunk::from_data(bytes)?),
                    crate::FetchResponse::Suggest(node_id) => {
                        // TODO connect to node
                        let _ = interaction.get_peer().net().connect_to(node_id, None).await;

                        Err(InteractionAsyncStoreError::NotFound)
                    }
                }
            } else {
                Err(InteractionAsyncStoreError::InvalidResponse)
            }
        })
    }

    fn put_encrypted<C: DataChunk>(&self, chunk: C) -> Promise<(), Self::Error> {
        let chunk = chunk.into_owned();
        let hash = chunk.hash();
        let mut interaction = self.clone();

        Promise::new(async move {
            interaction
                .send_packet(crate::Packet::PutRequest(crate::PutRequest {
                    data: chunk.into_bytes(),
                }))
                .await?;

            let packet = match interaction.next().await {
                None => return Err(InteractionAsyncStoreError::NoResponse)?,
                Some(Ok(packet)) => packet,
                Some(Err(err)) => Err(err)?,
            };

            if let Packet::PutResponse(response) = packet {
                match response {
                    crate::PutResponse::Failure => Err(InteractionAsyncStoreError::PutFailed),
                    crate::PutResponse::LimitExceeded => {
                        Err(InteractionAsyncStoreError::LimitExceeded)
                    }
                    crate::PutResponse::Success(hkey) => {
                        if hkey.as_bytes() == hash.as_bytes() {
                            Ok(())
                        } else {
                            Err(InteractionAsyncStoreError::HkeyMismatch {
                                expected: hash,
                                received: hkey,
                            })
                        }
                    }
                }
            } else {
                Err(InteractionAsyncStoreError::InvalidResponse)
            }
        })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum InteractionAsyncStoreError {
    #[error(transparent)]
    DataChunk(#[from] ps_datachunk::PsDataChunkError),
    #[error(transparent)]
    Hkey(#[from] PsHkeyError),
    #[error("The Peer returned an unexpected Hkey.")]
    HkeyMismatch {
        expected: Arc<Hash>,
        received: String,
    },
    #[error("Invalid response")]
    InvalidResponse,
    #[error("Exceeded usage limits")]
    LimitExceeded,
    #[error("Stream ended without response.")]
    NoResponse,
    #[error("Chunk not found")]
    NotFound,
    #[error("Promise already consumed.")]
    PromiseAlreadyConsumed,
    #[error("Peer responded to PutRequest with Failure.")]
    PutFailed,
    #[error(transparent)]
    ReadPacket(#[from] InteractionReadPacketError),
    #[error("Peer returned an Error.")]
    ReturnedError(String),
    #[error(transparent)]
    SendPacket(#[from] InteractionSendPacketError),
}

impl PromiseRejection for InteractionAsyncStoreError {
    fn already_consumed() -> Self {
        Self::PromiseAlreadyConsumed
    }
}
