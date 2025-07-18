use std::sync::Arc;

use anyhow::Result;

use crate::{NetState, Peer, PeerState, ScatterNet};

impl ScatterNet {
    #[allow(clippy::significant_drop_tightening)]
    pub fn get_state(&self) -> Result<NetState> {
        let peers: Vec<Arc<Peer>> = self.read().peers.values().cloned().collect();

        let peers: Vec<PeerState> = peers.iter().map(|peer| peer.get_state()).collect();

        let mut guard = self.write();
        let state = &mut guard.state;

        state.peers = peers;

        Ok(state.clone())
    }
}
