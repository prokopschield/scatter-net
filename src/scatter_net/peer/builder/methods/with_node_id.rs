use iroh::EndpointId;

use crate::PeerBuilder;

impl PeerBuilder {
    #[must_use]
    pub fn with_node_id(mut self, node_id: impl Into<EndpointId>) -> Self {
        self.node_id = node_id.into();

        self
    }
}
