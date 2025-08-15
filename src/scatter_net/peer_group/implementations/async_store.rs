use std::sync::Arc;

use ps_datachunk::{DataChunk, OwnedDataChunk, PsDataChunkError};
use ps_hash::Hash;
use ps_hkey::{AsyncStore, PsHkeyError};
use ps_promise::{Promise, PromiseRejection};

use crate::{distance, PeerGroup};

impl AsyncStore for PeerGroup {
    type Chunk = OwnedDataChunk;
    type Error = PeerGroupAsyncStoreError;

    fn get(&self, hash: &Hash) -> Promise<Self::Chunk, Self::Error> {
        let hash = Arc::new(*hash);
        let this = self.clone();

        Promise::new(async move {
            let peers = this.sort_peers(|lhs, rhs| {
                distance(lhs.node_id, &**hash).cmp(&distance(rhs.node_id, &**hash))
            });

            if peers.is_empty() {
                return Err(Self::Error::NoPeers);
            }

            let mut errors = Vec::new();

            for peer in peers {
                match peer.get(&hash).await {
                    Ok(chunk) => return Ok(chunk),
                    Err(err) => errors.push(err),
                }
            }

            Err(Self::Error::AllFailed(errors))
        })
    }

    fn put_encrypted<C: DataChunk>(&self, chunk: C) -> Promise<(), Self::Error> {
        let chunk = chunk.into_owned();
        let hash = chunk.hash();
        let this = self.clone();

        Promise::new(async move {
            let peers = this.sort_peers(|lhs, rhs| {
                distance(lhs.node_id, &**hash).cmp(&distance(rhs.node_id, &**hash))
            });

            if peers.is_empty() {
                return Err(Self::Error::NoPeers);
            }

            let mut errors = Vec::new();

            for peer in peers {
                match peer.put_encrypted(chunk.clone()).await {
                    Ok(()) => return Ok(()),
                    Err(err) => errors.push(err),
                }
            }

            Err(Self::Error::AllFailed(errors))
        })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PeerGroupAsyncStoreError {
    #[error("All Peer calls failed: {0:?}")]
    AllFailed(Vec<crate::PeerAsyncStoreError>),
    #[error("This Promise was consumed more than once.")]
    AlreadyConsumed,
    #[error("DataChunk error: {0}")]
    DataChunk(#[from] PsDataChunkError),
    #[error("Hkey error: {0}")]
    Hkey(#[from] PsHkeyError),
    #[error("This PeerGroup has no active Peers.")]
    NoPeers,
}

impl PromiseRejection for PeerGroupAsyncStoreError {
    fn already_consumed() -> Self {
        Self::AlreadyConsumed
    }
}
