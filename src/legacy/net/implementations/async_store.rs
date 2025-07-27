use std::sync::Arc;

use ps_datachunk::{PsDataChunkError, SerializedDataChunk};
use ps_hkey::{AsyncStore, PsHkeyError};
use ps_promise::{Promise, PromiseRejection};

use crate::{ScatterNet, ScatterNetFetchEncryptedChunkError, ScatterNetPutEncryptedError};

impl AsyncStore for ScatterNet {
    type Chunk = SerializedDataChunk;
    type Error = AsyncStoreError;

    fn get(&self, hash: &ps_hash::Hash) -> ps_promise::Promise<Self::Chunk, Self::Error> {
        let net = self.clone();
        let hash = Arc::from(*hash);

        Promise::new(async move { Ok(net.fetch_encrypted_chunk(hash).await?) })
    }

    fn put_encrypted<C: ps_datachunk::DataChunk>(&self, chunk: C) -> Promise<(), Self::Error> {
        let net = self.clone();
        let chunk = chunk.into_owned();

        Promise::new(async move { net.put_encrypted(chunk.data_ref())?.await?;Ok(()) })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AsyncStoreError {
    #[error("This Promise was consumed more than once.")]
    AlreadyConsumed,
    #[error(transparent)]
    DataChunk(#[from] PsDataChunkError),
    #[error(transparent)]
    Fetch(#[from] ScatterNetFetchEncryptedChunkError),
    #[error(transparent)]
    Hkey(#[from] PsHkeyError),
    #[error(transparent)]
    Put(#[from] ScatterNetPutEncryptedError),
}

impl PromiseRejection for AsyncStoreError {
    fn already_consumed() -> Self {
        Self::AlreadyConsumed
    }
}
