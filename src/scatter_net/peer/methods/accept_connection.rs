use std::sync::Arc;

use anyhow::Result;
use iroh::endpoint::Incoming;

use crate::{Peer, ScatterNet};

impl Peer {
    pub async fn accept_connection(net: Arc<ScatterNet>, incoming: Incoming) -> Result<Self> {
        let connection = incoming.accept()?.await?;

        Ok(Self::new(net, connection))
    }
}
