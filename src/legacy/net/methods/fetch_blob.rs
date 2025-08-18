use ps_buffer::{BufferError, SharedBuffer, ToSharedBuffer};
use ps_hkey::Hkey;

use crate::ScatterNet;

impl ScatterNet {
    pub async fn fetch_blob(self, hkey: &Hkey) -> Result<SharedBuffer, ScatterNetFetchBlobError> {
        let buffer = hkey
            .resolve_async(&self)
            .await?
            .data_ref()
            .to_shared_buffer()?;

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
