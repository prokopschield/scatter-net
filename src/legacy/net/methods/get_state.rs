use anyhow::Result;

use crate::{NetState, Peer, PeerState, ScatterNet};

impl ScatterNet {
    #[allow(clippy::significant_drop_tightening)]
    pub fn get_state(&self) -> Result<NetState> {
        let peers: Vec<Peer> = self.read().peers.values().cloned().collect();

        let peers: Vec<PeerState> = peers.iter().map(Peer::get_state).collect();

        let mut guard = self.write();
        let state = &mut guard.state;

        state.peers = peers;

        Ok(state.clone())
    }
}
