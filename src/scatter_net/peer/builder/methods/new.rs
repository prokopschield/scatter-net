use iroh::NodeAddr;

use crate::{PeerBuilder, ScatterNet};

impl PeerBuilder {
    #[must_use]
    pub fn new(net: ScatterNet, node_addr: impl Into<NodeAddr>) -> Self {
        let NodeAddr {
            node_id,
            relay_url,
            direct_addresses,
        } = node_addr.into();

        Self {
            direct_addresses,
            net,
            node_id,
            peer_group: None,
            relay_url,
            state: None,
        }
    }
}
