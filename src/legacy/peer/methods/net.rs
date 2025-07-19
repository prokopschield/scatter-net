use crate::{Peer, ScatterNet};

impl Peer {
    pub const fn net(&self) -> &ScatterNet {
        &self.net
    }
}
