use crate::{PeerBuilder, PeerGroup};

impl PeerBuilder {
    #[must_use]
    pub fn with_peer_group(mut self, peer_group: PeerGroup) -> Self {
        self.peer_group = Some(peer_group);

        self
    }
}
