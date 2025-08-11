use iroh::NodeId;

use crate::PeerBuilder;

impl PeerBuilder {
    #[must_use]
    pub fn with_node_id(mut self, node_id: impl Into<NodeId>) -> Self {
        self.node_addr.node_id = node_id.into();

        self
    }
}
