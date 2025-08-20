use std::sync::Arc;

use ps_datachunk::{OwnedDataChunk, PsDataChunkError};
use ps_hkey::{AsyncStore, PsHkeyError};
use ps_promise::{Promise, PromiseRejection};

use crate::{FetchError, ScatterNet};

impl AsyncStore for ScatterNet {
    type Chunk = OwnedDataChunk;
    type Error = ScatterNetAsyncStoreError;

    fn get(&self, hash: &ps_hash::Hash) -> ps_promise::Promise<Self::Chunk, Self::Error> {
        let fetch = self.upsert_fetch(Arc::new(*hash));

        Promise::new(async move { Ok(fetch.await?) })
    }

    fn put_encrypted<C: ps_datachunk::DataChunk>(
        &self,
        chunk: C,
    ) -> ps_promise::Promise<(), Self::Error> {
        self.upsert_put(chunk.into_owned());

        Promise::resolve(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ScatterNetAsyncStoreError {
    #[error("Promise was consumed more than once.")]
    AlreadyConsumed,
    #[error("DataChunk error: {0}")]
    DataChunk(#[from] PsDataChunkError),
    #[error(transparent)]
    Fetch(#[from] FetchError),
    #[error("Hkey error: {0}")]
    Hkey(#[from] PsHkeyError),
    #[error(transparent)]
    Put(#[from] crate::PutError),
}

impl PromiseRejection for ScatterNetAsyncStoreError {
    fn already_consumed() -> Self {
        Self::AlreadyConsumed
    }
}
