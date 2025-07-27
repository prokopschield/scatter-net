use ps_buffer::{BufferError, SharedBuffer, ToSharedBuffer};
use ps_datachunk::SerializedDataChunk;
use ps_hkey::Hkey;

use crate::{AsyncStoreError, ScatterNet};

impl ScatterNet {
    pub async fn fetch_blob(self, hkey: &Hkey) -> Result<SharedBuffer, ScatterNetFetchBlobError> {
        let buffer = hkey
            .resolve_async::<SerializedDataChunk, AsyncStoreError, Self>(&self)
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
    Buffer(#[from] BufferError),
}
