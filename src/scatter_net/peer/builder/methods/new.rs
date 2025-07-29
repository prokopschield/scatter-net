use iroh::NodeAddr;

use crate::{PeerBuilder, ScatterNet};

impl PeerBuilder {
    #[must_use]
    pub fn new(net: ScatterNet, node_addr: impl Into<NodeAddr>) -> Self {
        Self {
            net,
            node_addr: node_addr.into(),
            peer_group: None,
            state: None,
        }
    }
}
