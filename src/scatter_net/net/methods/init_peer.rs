use std::sync::Arc;

use anyhow::Result;
use iroh::endpoint::Connection;

use crate::{Peer, PeerState, ScatterNet};

impl ScatterNet {
    pub fn init_peer(
        net: &Arc<Self>,
        connection: Connection,
        state: Option<PeerState>,
    ) -> Result<Arc<Peer>> {
        let node_id = connection.remote_node_id()?;
        let mut peers_guard = net.peers.write();

        if let Some(peer) = peers_guard.get(&node_id) {
            // TODO peer.replace_connection(connection).await?;

            return Ok(peer.clone());
        }

        let peer = Peer::init(net.clone(), connection, state)?;

        peers_guard.insert(node_id, peer.clone());

        drop(peers_guard);

        // TODO put peer into PeerGroup

        Ok(peer)
    }
}
