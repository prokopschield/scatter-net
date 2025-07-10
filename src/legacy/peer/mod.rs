mod implementations;
mod methods;
mod types;

use std::sync::Arc;

use iroh::{endpoint::Connection, NodeId};
pub use methods::*;
use parking_lot::RwLock;
pub use types::*;

use super::ScatterNet;

#[derive(Debug)]
pub struct Peer {
    connection: RwLock<Connection>,
    net: Arc<ScatterNet>,
    node_id: NodeId,
    state: Arc<RwLock<PeerState>>,
}
