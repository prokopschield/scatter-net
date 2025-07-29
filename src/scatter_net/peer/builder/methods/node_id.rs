use iroh::NodeId;

use crate::PeerBuilder;

impl PeerBuilder {
    #[must_use]
    pub const fn node_id(mut self, node_id: NodeId) -> Self {
        self.node_id = node_id;

        self
    }
}
