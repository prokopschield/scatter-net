use std::sync::Arc;

use anyhow::Result;
use iroh::endpoint::Incoming;

use crate::{Peer, ScatterNet};

impl Peer {
    pub async fn accept_connection(net: Arc<ScatterNet>, incoming: Incoming) -> Result<Arc<Self>> {
        let connection = incoming.accept()?.await?;

        Self::init(net, connection, None)
    }
}
