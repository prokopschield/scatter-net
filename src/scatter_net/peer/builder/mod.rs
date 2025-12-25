mod methods;

use std::collections::BTreeSet;

use iroh::TransportAddr;

use crate::{PeerGroup, ScatterNet};

use super::PeerState;

#[derive(Clone, Debug)]
pub struct PeerBuilder {
    direct_addresses: BTreeSet<TransportAddr>,
    net: ScatterNet,
    node_id: iroh::EndpointId,
    peer_group: Option<PeerGroup>,
    state: Option<PeerState>,
}
