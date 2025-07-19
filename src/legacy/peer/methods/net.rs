use crate::{Peer, ScatterNet};

impl Peer {
    #[must_use]
    pub fn net(&self) -> &ScatterNet {
        &self.net
    }
}
