use std::sync::Arc;

use crate::{spawn_and_forget, Peer};

impl Peer {
    pub fn listen(peer: Arc<Self>) {
        spawn_and_forget(Self::listen_uni(peer.clone()));
        spawn_and_forget(Self::listen_bi(peer.clone()));
        spawn_and_forget(Self::listen_dg(peer));
    }
}
