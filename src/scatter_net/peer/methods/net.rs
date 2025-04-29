use std::sync::Arc;

use crate::{Peer, ScatterNet};

impl Peer {
    pub const fn net(&self) -> &Arc<ScatterNet> {
        &self.net
    }
}
