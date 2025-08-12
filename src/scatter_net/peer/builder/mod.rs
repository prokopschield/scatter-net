mod methods;

use std::{collections::BTreeSet, net::SocketAddr};

pub use methods::*;

use crate::{PeerGroup, ScatterNet};

use super::PeerState;

#[derive(Clone, Debug)]
pub struct PeerBuilder {
    direct_addresses: BTreeSet<SocketAddr>,
    net: ScatterNet,
    node_id: iroh::NodeId,
    relay_url: Option<iroh::RelayUrl>,
    peer_group: Option<PeerGroup>,
    state: Option<PeerState>,
}
