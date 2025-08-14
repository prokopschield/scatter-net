use std::cmp::Ordering;

use crate::{Peer, PeerGroup};

impl PeerGroup {
    pub fn sort_peers<F>(&self, compare: F) -> Vec<Peer>
    where
        F: FnMut(&Peer, &Peer) -> Ordering,
    {
        let mut peers: Vec<Peer> = self.read().peers.values().cloned().collect();

        peers.sort_by(compare);

        peers
    }
}
