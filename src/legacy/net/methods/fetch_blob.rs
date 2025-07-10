use std::sync::Arc;

use anyhow::{Error, Result};
use ps_buffer::{SharedBuffer, ToSharedBuffer};
use ps_datachunk::{PsDataChunkError, SerializedDataChunk};
use ps_hkey::{Hkey, PsHkeyError};

use crate::{AsyncStoreError, ScatterNet};

impl ScatterNet {
    pub async fn fetch_blob(self: Arc<Self>, hkey: &Hkey) -> Result<SharedBuffer> {
        let buffer = hkey
            .resolve_async::<SerializedDataChunk, Error, AsyncStoreError, Self>(&*self)
            .await?
            .data_ref()
            .to_shared_buffer()?;

        Ok(buffer)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ScatterNetFetchBlobError {
    #[error(transparent)]
    AsyncStore(#[from] AsyncStoreError),
    #[error(transparent)]
    DataChunk(#[from] PsDataChunkError),
    #[error(transparent)]
    Fetch(#[from] crate::ScatterNetFetchEncryptedChunkError),
    #[error(transparent)]
    Hkey(#[from] PsHkeyError),
    #[error("Chunk was not found.")]
    NotFound,
}
