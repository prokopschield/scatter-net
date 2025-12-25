use iroh::EndpointId;

use crate::{Peer, ScatterNet};

impl ScatterNet {
    #[must_use]
    pub fn get_peer(&self, node_id: &EndpointId) -> Option<Peer> {
        self.read().peers.get(node_id).cloned()
    }
}
