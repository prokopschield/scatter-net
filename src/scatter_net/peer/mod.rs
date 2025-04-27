mod implementations;
mod methods;
mod types;

pub use methods::*;

use std::sync::Arc;

use iroh::endpoint::Connection;
use parking_lot::RwLock;
pub use types::*;

use super::ScatterNet;

#[derive(Debug)]
pub struct Peer {
    connection: RwLock<Connection>,
    net: Arc<ScatterNet>,
    state: Arc<RwLock<PeerState>>,
}
