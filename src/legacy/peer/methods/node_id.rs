use iroh::NodeId;

use crate::Peer;

impl Peer {
    #[must_use]
    pub fn node_id(&self) -> NodeId {
        self.node_id
    }
}
