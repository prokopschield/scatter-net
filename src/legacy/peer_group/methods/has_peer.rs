use crate::{Peer, PeerGroup};

impl PeerGroup {
    #[must_use]
    pub fn has_peer(&self, peer: &Peer) -> bool {
        self.peers.read().contains_key(&peer.node_id())
    }
}
