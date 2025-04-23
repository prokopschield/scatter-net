mod methods;

use std::{collections::HashMap, sync::Arc};

use iroh::NodeId;
use parking_lot::RwLock;

use super::{peer::Peer, ScatterNet};

#[derive(Clone, Debug, Default)]
pub struct PeerGroupState {
    terminated: bool,
}

#[derive(Clone, Debug)]
pub struct PeerGroup {
    net: Arc<ScatterNet>,
    peers: Arc<RwLock<HashMap<NodeId, Arc<Peer>>>>,
    state: Arc<RwLock<PeerGroupState>>,
}
