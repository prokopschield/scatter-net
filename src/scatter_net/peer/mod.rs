mod implementations;
mod methods;
mod types;

use std::sync::Arc;

use iroh::endpoint::Connection;
use parking_lot::RwLock;
pub use types::*;

use super::ScatterNet;

#[derive(Clone, Debug)]
pub struct Peer {
    connection: Connection,
    net: Arc<ScatterNet>,
    state: Arc<RwLock<PeerState>>,
}
