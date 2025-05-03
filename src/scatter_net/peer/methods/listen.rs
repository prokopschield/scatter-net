use std::sync::Arc;

use crate::{spawn_and_forget, Peer};

impl Peer {
    pub fn listen(self: Arc<Self>) {
        spawn_and_forget(Self::listen_uni(self.clone()));
        spawn_and_forget(Self::listen_bi(self.clone()));
        spawn_and_forget(Self::listen_dg(self));
    }
}
