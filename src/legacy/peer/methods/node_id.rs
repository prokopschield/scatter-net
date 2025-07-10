use iroh::NodeId;

use crate::Peer;

impl Peer {
    pub const fn node_id(&self) -> NodeId {
        self.node_id
    }
}
