use crate::{Peer, PeerGroup};

impl PeerGroup {
    pub fn has_peer(&self, peer: &Peer) -> bool {
        self.peers.read().contains_key(&peer.node_id())
    }
}
