use iroh::NodeAddr;

use crate::PeerBuilder;

impl PeerBuilder {
    #[must_use]
    pub fn node_addr(mut self, node_addr: impl Into<NodeAddr>) -> Self {
        self.node_addr = node_addr.into();

        self
    }
}
