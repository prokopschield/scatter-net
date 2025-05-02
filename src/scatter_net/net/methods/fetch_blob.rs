use std::sync::Arc;

use anyhow::Result;
use ps_buffer::{SharedBuffer, ToSharedBuffer};
use ps_datachunk::{PsDataChunkError, SerializedDataChunk};
use ps_hkey::{Hash, Hkey, PsHkeyError};

use crate::ScatterNet;

impl ScatterNet {
    pub async fn fetch_blob(net: Arc<Self>, hkey: &Hkey) -> Result<SharedBuffer> {
        let resolver = move |hash: &Hash| {
            let net_clone = net.clone();
            let hash_arc = Arc::new(*hash);

            async move { Ok(net_clone.fetch_encrypted_chunk(hash_arc).await?) }
        };

        let resolved = hkey
            .resolve_async::<SerializedDataChunk, ScatterNetFetchBlobError, _, _>(&resolver)
            .await?;

        Ok(resolved.data_ref().to_shared_buffer()?)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ScatterNetFetchBlobError {
    #[error(transparent)]
    DataChunk(#[from] PsDataChunkError),
    #[error(transparent)]
    Fetch(#[from] crate::ScatterNetFetchEncryptedChunkError),
    #[error(transparent)]
    Hkey(#[from] PsHkeyError),
    #[error("Chunk was not found.")]
    NotFound,
}
