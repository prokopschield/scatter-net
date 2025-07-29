use crate::PeerBuilder;

impl PeerBuilder {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            net: None,
            node_id: None,
            peer_group: None,
            state: None,
        }
    }
}
