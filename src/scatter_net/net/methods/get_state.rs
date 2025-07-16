use std::sync::Arc;

use anyhow::Result;

use crate::{NetState, Peer, PeerState, ScatterNet};

impl ScatterNet {
    pub fn get_state(&self) -> Result<NetState> {
        let peers: Vec<Arc<Peer>> = self
            .peers
            .read().values().cloned()
            .collect();

        let peers: Vec<PeerState> = peers.iter().map(|peer| peer.get_state()).collect();

        let mut state = self.state.write();

        state.peers = peers;

        Ok(state.clone())
    }
}
