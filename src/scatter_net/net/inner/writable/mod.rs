use std::{collections::HashMap, sync::Arc};

use iroh::NodeId;
use ps_hash::Hash;

use crate::{Fetch, NetState, Peer, PeerGroup, Put, ScatterNetPutBlob};

#[derive(Clone, Debug, Default)]
pub struct ScatterNetInnerWritable {
    pub fetches: HashMap<Arc<Hash>, Fetch>,
    pub peers: HashMap<NodeId, Peer>,
    pub peer_groups: Vec<PeerGroup>,
    pub put_cache: HashMap<Arc<Hash>, ScatterNetPutBlob>,
    pub puts: HashMap<Arc<Hash>, Put>,
    pub state: NetState,
}
