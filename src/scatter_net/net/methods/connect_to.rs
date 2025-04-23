use std::sync::Arc;

use anyhow::Result;
use iroh::NodeId;

use crate::{Peer, ScatterNet, ALPN};

impl ScatterNet {
    pub async fn connect_to(net: &Arc<Self>, node_id: NodeId) -> Result<Arc<Peer>> {
        let connection = net.endpoint.connect(node_id, ALPN).await?;

        Self::init_peer(net, connection)
    }
}
