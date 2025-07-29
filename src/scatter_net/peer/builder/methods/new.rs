use iroh::NodeId;

use crate::{PeerBuilder, ScatterNet};

impl PeerBuilder {
    #[must_use]
    pub const fn new(net: ScatterNet, node_id: NodeId) -> Self {
        Self {
            net,
            node_id,
            peer_group: None,
            state: None,
        }
    }
}
