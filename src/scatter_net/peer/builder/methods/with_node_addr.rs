use iroh::NodeAddr;

use crate::PeerBuilder;

impl PeerBuilder {
    #[must_use]
    pub fn with_node_addr(mut self, node_addr: impl Into<NodeAddr>) -> Self {
        let NodeAddr {
            node_id,
            relay_url,
            direct_addresses,
        } = node_addr.into();

        self.direct_addresses = direct_addresses;
        self.node_id = node_id;
        self.relay_url = relay_url;

        self
    }
}
