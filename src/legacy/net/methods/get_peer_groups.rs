use std::sync::Arc;

use crate::{PeerGroup, ScatterNet};

impl ScatterNet {
    #[must_use]
    pub fn get_peer_groups(&self) -> Vec<Arc<PeerGroup>> {
        self.peer_groups.read().clone()
    }
}
