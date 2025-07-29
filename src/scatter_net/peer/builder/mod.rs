mod methods;

use iroh::NodeId;

use crate::{PeerGroup, ScatterNet};

use super::PeerState;

#[derive(Clone, Debug)]
pub struct PeerBuilder {
    net: ScatterNet,
    node_id: NodeId,
    peer_group: Option<PeerGroup>,
    state: Option<PeerState>,
}
