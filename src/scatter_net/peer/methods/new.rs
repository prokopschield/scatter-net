use std::sync::Arc;

use iroh::endpoint::Connection;

use crate::{Peer, ScatterNet};

impl Peer {
    #[must_use]
    pub fn new(net: Arc<ScatterNet>, connection: Connection) -> Self {
        Self {
            connection,
            net,
            state: Arc::default(),
        }
    }
}
