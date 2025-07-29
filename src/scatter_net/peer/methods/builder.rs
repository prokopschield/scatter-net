use iroh::NodeId;

use crate::{Peer, PeerBuilder, ScatterNet};

impl Peer {
    #[must_use]
    pub const fn builder(net: ScatterNet, node_id: NodeId) -> PeerBuilder {
        PeerBuilder::new(net, node_id)
    }
}
