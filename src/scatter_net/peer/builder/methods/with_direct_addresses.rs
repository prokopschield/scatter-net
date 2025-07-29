use std::net::SocketAddr;

use crate::PeerBuilder;

impl PeerBuilder {
    #[must_use]
    pub fn with_direct_addresses(
        mut self,
        addresses: impl IntoIterator<Item = SocketAddr>,
    ) -> Self {
        self.node_addr = self.node_addr.with_direct_addresses(addresses);

        self
    }
}
