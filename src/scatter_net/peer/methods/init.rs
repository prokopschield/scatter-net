use std::sync::Arc;

use anyhow::Result;
use iroh::endpoint::Connection;
use parking_lot::RwLock;

use crate::{scatter_net::peer::PeerState, Peer, ScatterNet};

impl Peer {
    pub fn init(
        net: Arc<ScatterNet>,
        connection: Connection,
        state: Option<PeerState>,
    ) -> Result<Arc<Self>> {
        let mut state = state.unwrap_or(PeerState {
            node_id: connection.remote_node_id()?,
            terminated: false,
        });

        state.terminated = false;

        let peer = Self {
            connection: RwLock::new(connection),
            net,
            state: Arc::new(RwLock::new(state)),
        };

        Ok(Arc::from(peer))
    }
}
