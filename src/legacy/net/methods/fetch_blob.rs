use bytes::Bytes;
use ps_buffer::BufferError;
use ps_hkey::Hkey;

use crate::ScatterNet;

impl ScatterNet {
    pub async fn fetch_blob(self, hkey: &Hkey) -> Result<Bytes, ScatterNetFetchBlobError> {
        let buffer = hkey.resolve_async(&self).await?;

        Ok(buffer)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ScatterNetFetchBlobError {
    #[error(transparent)]
    AsyncStore(#[from] crate::ScatterNetAsyncStoreError),
    #[error(transparent)]
    Buffer(#[from] BufferError),
}
