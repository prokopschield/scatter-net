use crate::{Peer, ScatterNet};

impl ScatterNet {
    #[must_use]
    pub fn known_peers(&self) -> Vec<Peer> {
        self.read().peers.values().cloned().collect()
    }
}
