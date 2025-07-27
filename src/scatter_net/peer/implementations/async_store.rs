use std::sync::Arc;

use ps_datachunk::{OwnedDataChunk, PsDataChunkError};
use ps_hkey::{AsyncStore, PsHkeyError};
use ps_promise::{Promise, PromiseRejection};

use crate::{InteractionAsyncStoreError, Peer, PeerBeginInteractionError};

impl AsyncStore for Peer {
    type Chunk = OwnedDataChunk;
    type Error = PeerAsyncStoreError;

    fn get(&self, hash: &ps_hash::Hash) -> ps_promise::Promise<Self::Chunk, Self::Error> {
        let peer = self.clone();
        let hash = Arc::from(*hash);

        Promise::new(async move {
            let interaction = peer.begin_interaction().await?;

            Ok(interaction.get(&hash).await?)
        })
    }

    fn put_encrypted<C: ps_datachunk::DataChunk>(&self, chunk: C) -> Promise<(), Self::Error> {
        let peer = self.clone();
        let chunk = chunk.into_owned();

        Promise::new(async move {
            let interaction = peer.begin_interaction().await?;

            Ok(interaction.put_encrypted(chunk).await?)
        })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PeerAsyncStoreError {
    #[error("This Promise was already consumed")]
    AlreadyConsumed,
    #[error(transparent)]
    BeginInteraction(#[from] PeerBeginInteractionError),
    #[error(transparent)]
    DataChunk(#[from] PsDataChunkError),
    #[error(transparent)]
    Hkey(#[from] PsHkeyError),
    #[error(transparent)]
    Interaction(#[from] InteractionAsyncStoreError),
}

impl PromiseRejection for PeerAsyncStoreError {
    fn already_consumed() -> Self {
        Self::AlreadyConsumed
    }
}
