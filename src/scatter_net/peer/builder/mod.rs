mod methods;

pub use methods::*;

use crate::{PeerGroup, ScatterNet};

use super::PeerState;

#[derive(Clone, Debug)]
pub struct PeerBuilder {
    connection: Option<iroh::endpoint::Connection>,
    net: ScatterNet,
    node_addr: iroh::NodeAddr,
    peer_group: Option<PeerGroup>,
    state: Option<PeerState>,
}
