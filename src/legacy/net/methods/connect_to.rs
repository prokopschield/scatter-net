use std::sync::Arc;

use anyhow::Result;
use iroh::NodeId;

use crate::{Peer, PeerState, ScatterNet, ALPN};

impl ScatterNet {
    pub async fn connect_to(&self, node_id: NodeId, state: Option<PeerState>) -> Result<Arc<Peer>> {
        eprintln!("Attempting connection to {node_id}");

        let connection = self.endpoint.connect(node_id, ALPN).await?;

        eprintln!("Connection established to {node_id}");

        Self::init_peer(self, connection, state)
    }
}
