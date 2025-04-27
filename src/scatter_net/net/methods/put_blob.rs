use std::sync::Arc;

use anyhow::Result;
use ps_hkey::Hkey;

use crate::ScatterNet;

impl ScatterNet {
    pub async fn put_blob(net: Arc<Self>, blob: &[u8]) -> Result<Hkey> {
        let hkey = net.lake.put_blob(blob)?;

        // TODO: upload blob to peers

        Ok(hkey)
    }
}
