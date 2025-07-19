use std::sync::Arc;

use bytes::Bytes;
use ps_hash::Hash;

use crate::{Packet, Peer};

impl Packet {
    pub async fn process(self, peer: Arc<Peer>) -> Result<Option<Self>, PacketProcessError> {
        use PacketProcessError::ReceivedErrorPacket;

        match self {
            Self::Empty | Self::Pong => Ok(None),
            Self::Error(str) => Err(ReceivedErrorPacket(str)),
            Self::Ping => Ok(Some(Self::Pong)),
            Self::FetchRequest(request) => Ok(Some(
                match peer
                    .net()
                    .fetch_encrypted_chunk(Arc::new(Hash::try_from(request.hash.as_bytes())?))
                    .await
                {
                    Ok(chunk) => {
                        Self::FetchResponse(crate::FetchResponse::Success(Bytes::from_owner(chunk)))
                    }
                    Err(crate::ScatterNetFetchEncryptedChunkError::NotFound) => {
                        Self::FetchResponse(crate::FetchResponse::NotFound)
                    }
                    Err(_) => Self::FetchResponse(crate::FetchResponse::Error),
                },
            )),
            Self::FetchResponse(response) => response.process(peer).await,
            Self::PutRequest(request) => Ok(Some(
                (peer.net().put_blob(request.data)?.early_return().await).map_or_else(
                    |_| Self::PutResponse(crate::PutResponse::Failure),
                    |hkey| Self::PutResponse(crate::PutResponse::Success(hkey.to_string())),
                ),
            )),
            Self::PutResponse(response) => {
                eprintln!("Received unsolicited PutResponse({response:?}) from {peer}");
                Ok(Some(Self::Error("Unsolicited PutResponse.".to_string())))
            }
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PacketProcessError {
    #[error(transparent)]
    HashValidation(#[from] ps_hash::HashValidationError),
    #[error(transparent)]
    Put(#[from] crate::ScatterNetPutBlobError),
    #[error("The Peer sent an Error packet.")]
    ReceivedErrorPacket(String),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
