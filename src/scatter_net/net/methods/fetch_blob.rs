use std::sync::Arc;

use anyhow::Result;
use ps_buffer::{SharedBuffer, ToSharedBuffer};
use ps_datalake::error::PsDataLakeError;
use ps_hkey::Hkey;

use crate::ScatterNet;

impl ScatterNet {
    pub async fn fetch_blob(net: Arc<Self>, hkey: &Hkey) -> Result<SharedBuffer> {
        match net.lake.get_chunk_by_hkey(hkey) {
            Ok(chunk) => return Ok(chunk.data_ref().to_shared_buffer()?),
            Err(PsDataLakeError::NotFound) => (),
            Err(err) => {
                eprintln!("Fetching chunk {hkey} from DataLake failed: {err:?}");
            }
        };

        todo!()
    }
}
