use iroh::NodeId;

use crate::PeerBuilder;

impl PeerBuilder {
    #[must_use]
    pub fn node_id(self, node_id: impl Into<NodeId>) -> Self {
        self.node_addr(node_id.into())
    }
}
