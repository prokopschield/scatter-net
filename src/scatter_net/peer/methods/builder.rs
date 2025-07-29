use crate::{Peer, PeerBuilder};

impl Peer {
    #[must_use]
    pub const fn builder() -> PeerBuilder {
        PeerBuilder::new()
    }
}
