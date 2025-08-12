use std::net::SocketAddr;

use crate::PeerBuilder;

impl PeerBuilder {
    #[must_use]
    pub fn with_direct_addresses(
        mut self,
        addresses: impl IntoIterator<Item = SocketAddr>,
    ) -> Self {
        self.direct_addresses = addresses.into_iter().collect();

        self
    }
}
