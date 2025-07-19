mod implementations;
mod methods;
mod types;

use std::{collections::HashMap, sync::Arc};

use iroh::NodeId;
use parking_lot::RwLock;
pub use types::*;

use crate::{Peer, ScatterNet};

#[derive(Clone, Debug)]
pub struct PeerGroup {
    config: PeerGroupConfig,
    net: ScatterNet,
    peers: Arc<RwLock<HashMap<NodeId, Peer>>>,
}
