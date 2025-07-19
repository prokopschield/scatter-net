use crate::{Peer, PeerState};

impl Peer {
    #[must_use]
    pub fn get_state(&self) -> PeerState {
        self.read().state.clone()
    }
}
