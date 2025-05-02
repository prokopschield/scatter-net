mod implementations;
mod methods;
mod types;

use std::{collections::HashMap, sync::Arc};

use iroh::{Endpoint, NodeId};
pub use methods::*;
use parking_lot::RwLock;
use ps_datalake::lake::DataLake;
pub use types::*;

use super::{peer::Peer, peer_group::PeerGroup};

#[derive(Clone)]
pub struct ScatterNet {
    config: NetConfig,
    endpoint: Endpoint,
    lake: Arc<DataLake<'static>>,
    node_id: NodeId,
    peers: Arc<RwLock<HashMap<NodeId, Arc<Peer>>>>,
    peer_groups: Arc<RwLock<Vec<Arc<PeerGroup>>>>,
    state: Arc<RwLock<NetState>>,
}
