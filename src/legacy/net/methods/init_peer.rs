use anyhow::Result;
use iroh::endpoint::Connection;

use crate::{spawn_and_forget, Peer, PeerState, ScatterNet};

impl ScatterNet {
    pub fn init_peer(&self, connection: Connection, state: Option<PeerState>) -> Result<Peer> {
        let node_id = connection.remote_node_id()?;

        if let Some(peer) = self.get_peer(&node_id) {
            peer.replace_connection(connection);

            return Ok(peer);
        }

        let peer = Peer::builder(self.clone(), node_id)
            .with_option_state(state)
            .finalize(connection);

        spawn_and_forget({
            let peer = peer.clone();

            async move { Ok(peer.select_peer_group().await?) }
        });

        Ok(peer)
    }
}
