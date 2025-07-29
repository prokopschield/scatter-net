use iroh::RelayUrl;

use crate::PeerBuilder;

impl PeerBuilder {
    #[must_use]
    pub fn with_relay_url(mut self, relay_url: impl Into<RelayUrl>) -> Self {
        self.node_addr = self.node_addr.with_relay_url(relay_url.into());

        self
    }
}
