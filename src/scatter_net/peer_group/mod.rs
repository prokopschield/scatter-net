mod methods;
mod types;

use std::{collections::HashMap, sync::Arc};

use iroh::NodeId;
use parking_lot::RwLock;
pub use types::*;

use super::{peer::Peer, ScatterNet};

#[derive(Clone, Debug)]
pub struct PeerGroup {
    config: PeerGroupConfig,
    net: Arc<ScatterNet>,
    peers: Arc<RwLock<HashMap<NodeId, Arc<Peer>>>>,
}
