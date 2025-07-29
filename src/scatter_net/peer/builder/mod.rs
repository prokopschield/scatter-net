mod methods;

use iroh::NodeId;

use crate::{PeerGroup, ScatterNet};

use super::PeerState;

#[derive(Clone, Debug, Default)]
pub struct PeerBuilder {
    net: Option<ScatterNet>,
    node_id: Option<NodeId>,
    peer_group: Option<PeerGroup>,
    state: Option<PeerState>,
}
