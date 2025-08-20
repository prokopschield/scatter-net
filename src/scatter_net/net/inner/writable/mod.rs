use std::{collections::HashMap, sync::Arc};

use iroh::NodeId;
use ps_hash::Hash;

use crate::{Fetch, NetState, Peer, PeerGroup, Put};

#[derive(Clone, Debug, Default)]
pub struct ScatterNetInnerWritable {
    pub fetches: HashMap<Arc<Hash>, Fetch>,
    pub peers: HashMap<NodeId, Peer>,
    pub peer_groups: Vec<PeerGroup>,
    pub puts: HashMap<Arc<Hash>, Put>,
    pub state: NetState,
}
