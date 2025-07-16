use std::sync::Arc;

use bytes::Bytes;
use ps_datachunk::SerializedDataChunk;
use ps_hkey::AsyncStore;
use ps_promise::{Promise, PromiseRejection};

use crate::{ScatterNet, ScatterNetFetchEncryptedChunkError, ScatterNetPutBlobError};

impl AsyncStore for ScatterNet {
    type Chunk = SerializedDataChunk;
    type Error = AsyncStoreError;

    fn get(&self, hash: &ps_hash::Hash) -> ps_promise::Promise<Self::Chunk, Self::Error> {
        let net = self.clone();
        let hash = Arc::from(*hash);

        Promise::new(async move { Ok(net.fetch_encrypted_chunk(hash).await?) })
    }

    fn put(&self, data: &[u8]) -> ps_promise::Promise<ps_hkey::Hkey, Self::Error> {
        let net = Arc::from(self.clone());
        let blob = Bytes::from_owner(Arc::from(data));

        Promise::new(async move { Ok(net.put_blob(blob)?.early_return().await?) })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AsyncStoreError {
    #[error("This Promise was consumed more than once.")]
    AlreadyConsumed,
    #[error(transparent)]
    Fetch(#[from] ScatterNetFetchEncryptedChunkError),
    #[error(transparent)]
    Put(#[from] ScatterNetPutBlobError),
}

impl PromiseRejection for AsyncStoreError {
    fn already_consumed() -> Self {
        Self::AlreadyConsumed
    }
}
