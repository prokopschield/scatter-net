use ps_hkey::Hash;

use crate::{distance, Peer, PeerGroup};

impl PeerGroup {
    #[must_use]
    pub fn get_peer_by_hash(&self, hash: &Hash) -> Option<Peer> {
        let mut sel_distance = u64::MAX;
        let mut selected = None;

        self.read().peers.iter().for_each(|(node_id, peer)| {
            if !peer.is_available() {
                return;
            }

            let iter_distance = distance(hash, node_id);

            if iter_distance < sel_distance {
                sel_distance = iter_distance;
                selected = Some(peer.clone());
            }
        });

        selected
    }
}
