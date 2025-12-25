use anyhow::Result;
use iroh::EndpointId;

use crate::{Peer, PeerState, ScatterNet, ALPN};

impl ScatterNet {
    pub async fn connect_to(&self, node_id: EndpointId, state: Option<PeerState>) -> Result<Peer> {
        eprintln!("Attempting connection to {node_id}");

        let connection = self.endpoint.connect(node_id, ALPN).await?;

        eprintln!("Connection established to {node_id}");

        Self::init_peer(self, connection, state)
    }
}
