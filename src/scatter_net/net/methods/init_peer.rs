use std::sync::Arc;

use anyhow::Result;
use iroh::endpoint::Connection;

use crate::{Peer, ScatterNet};

impl ScatterNet {
    pub fn init_peer(net: &Arc<Self>, connection: Connection) -> Result<Arc<Peer>> {
        let node_id = connection.remote_node_id()?;
        let peer = Arc::from(Peer::new(net.clone(), connection));

        net.peers.write().insert(node_id, peer.clone());

        // TODO put peer into PeerGroup

        Ok(peer)
    }
}
