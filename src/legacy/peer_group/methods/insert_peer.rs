use crate::{Peer, PeerGroup};

impl PeerGroup {
    /// Inserts a [`Peer`] into this [`PeerGroup`]
    pub fn insert_peer(&self, peer: Peer) {
        self.write().peers.insert(peer.node_id(), peer);
    }
}
