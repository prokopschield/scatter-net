use iroh::NodeId;

use crate::{Peer, ScatterNet};

impl ScatterNet {
    #[must_use]
    pub fn get_peer(&self, node_id: &NodeId) -> Option<Peer> {
        self.read().peers.get(node_id).cloned()
    }
}
