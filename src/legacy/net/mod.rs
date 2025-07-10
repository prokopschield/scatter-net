mod config;
mod implementations;
mod methods;
mod types;

use std::{collections::HashMap, sync::Arc};

pub use config::*;
use iroh::{Endpoint, NodeId};
pub use methods::*;
use parking_lot::RwLock;
use ps_datalake::lake::DataLake;
use ps_hash::Hash;
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
    put_cache: Arc<RwLock<HashMap<Arc<Hash>, ScatterNetPutBlob>>>,
    state: Arc<RwLock<NetState>>,
}
