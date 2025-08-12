use anyhow::Result;
use iroh::endpoint::Connection;

use crate::{spawn_and_forget, Peer, PeerState, ScatterNet};

impl ScatterNet {
    pub fn init_peer(&self, connection: Connection, state: Option<PeerState>) -> Result<Peer> {
        let node_id = connection.remote_node_id()?;
        let mut guard = self.write();

        if let Some(peer) = guard.peers.get(&node_id) {
            peer.replace_connection(connection);

            return Ok(peer.clone());
        }

        let peer = Peer::builder(self.clone(), node_id)
            .with_option_state(state)
            .finalize(connection);

        guard.peers.insert(node_id, peer.clone());

        drop(guard);

        spawn_and_forget({
            let peer = peer.clone();

            async move { Ok(peer.select_peer_group().await?) }
        });

        Ok(peer)
    }
}
