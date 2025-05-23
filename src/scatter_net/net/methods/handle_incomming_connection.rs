use std::sync::Arc;

use anyhow::Result;
use iroh::endpoint::Incoming;

use crate::{Peer, ScatterNet};

impl ScatterNet {
    pub async fn handle_incoming_connection(
        self: Arc<Self>,
        incoming: Incoming,
    ) -> Result<Arc<Peer>> {
        let connection = incoming.accept()?.await?;

        Self::init_peer(&self, connection, None)
    }
}
