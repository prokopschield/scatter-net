use std::sync::Arc;

use anyhow::Result;
use iroh::endpoint::Connection;
use parking_lot::RwLock;

use crate::{legacy::peer::PeerState, Peer, PeerUsage, ScatterNet};

impl Peer {
    pub fn init(
        net: Arc<ScatterNet>,
        connection: Connection,
        state: Option<PeerState>,
    ) -> Result<Arc<Self>> {
        let node_id = connection.remote_node_id()?;

        let mut state = state.unwrap_or_else(|| PeerState {
            node_id,
            terminated: false,
            usage: PeerUsage::default(),
        });

        state.terminated = false;

        let peer = Self {
            connection: RwLock::new(connection),
            net,
            node_id,
            state: Arc::new(RwLock::new(state)),
        };

        let peer = Arc::from(peer);

        peer.clone().listen();

        Ok(peer)
    }
}
