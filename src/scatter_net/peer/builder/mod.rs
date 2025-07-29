mod methods;

use iroh::NodeAddr;

use crate::{PeerGroup, ScatterNet};

use super::PeerState;

#[derive(Clone, Debug)]
pub struct PeerBuilder {
    net: ScatterNet,
    node_addr: NodeAddr,
    peer_group: Option<PeerGroup>,
    state: Option<PeerState>,
}
