use crate::{PeerGroup, ScatterNet};

impl ScatterNet {
    #[must_use]
    pub fn get_peer_groups(&self) -> Vec<PeerGroup> {
        self.read().peer_groups.clone()
    }
}
