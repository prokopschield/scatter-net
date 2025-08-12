use iroh::RelayUrl;

use crate::PeerBuilder;

impl PeerBuilder {
    #[must_use]
    pub fn with_relay_url(mut self, relay_url: impl Into<RelayUrl>) -> Self {
        self.relay_url = Some(relay_url.into());

        self
    }
}
