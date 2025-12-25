use iroh::EndpointAddr;

use crate::PeerBuilder;

impl PeerBuilder {
    #[must_use]
    pub fn with_node_addr(mut self, node_addr: impl Into<EndpointAddr>) -> Self {
        let EndpointAddr { id, addrs } = node_addr.into();

        self.direct_addresses = addrs;
        self.node_id = id;

        self
    }
}
