use iroh::endpoint::VarInt;

use crate::{Peer, PeerGroup, ScatterNet, Terminate};

impl<E, R> Terminate<E, R> for ScatterNet
where
    E: Into<VarInt> + Send,
    R: AsRef<[u8]> + Send,
{
    fn terminate(&self, error_code: E, reason: &R) {
        let error_code = error_code.into();
        let reason = reason.as_ref();

        let peer_groups = self.read().peer_groups.clone();

        for peer_group in peer_groups {
            PeerGroup::terminate(&peer_group, error_code, &reason);
        }

        let peers: Vec<Peer> = self.read().peers.values().cloned().collect();

        for peer in peers {
            Peer::terminate(&peer, error_code, &reason);
        }
    }
}
