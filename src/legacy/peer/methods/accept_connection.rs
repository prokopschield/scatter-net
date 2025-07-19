use anyhow::Result;
use iroh::endpoint::Incoming;

use crate::{Peer, ScatterNet};

impl Peer {
    pub async fn accept_connection(net: ScatterNet, incoming: Incoming) -> Result<Self> {
        let connection = incoming.accept()?.await?;

        Self::init(net, connection, None)
    }
}
