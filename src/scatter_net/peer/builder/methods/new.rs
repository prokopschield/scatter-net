use iroh::EndpointAddr;

use crate::{PeerBuilder, ScatterNet};

impl PeerBuilder {
    #[must_use]
    pub fn new(net: ScatterNet, node_addr: impl Into<EndpointAddr>) -> Self {
        let EndpointAddr { id, addrs } = node_addr.into();

        Self {
            direct_addresses: addrs,
            net,
            node_id: id,
            peer_group: None,
            state: None,
        }
    }
}
