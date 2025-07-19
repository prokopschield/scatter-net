use anyhow::Result;
use iroh::endpoint::Incoming;

use crate::{Peer, ScatterNet};

impl ScatterNet {
    pub async fn handle_incoming_connection(self, incoming: Incoming) -> Result<Peer> {
        let connection = incoming.accept()?.await?;

        Self::init_peer(&self, connection, None)
    }
}
