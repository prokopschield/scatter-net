use iroh::EndpointAddr;

use crate::{Peer, PeerBuilder, ScatterNet};

impl Peer {
    #[must_use]
    pub fn builder(net: ScatterNet, node_addr: impl Into<EndpointAddr>) -> PeerBuilder {
        PeerBuilder::new(net, node_addr.into())
    }
}
