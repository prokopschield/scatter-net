use std::sync::Arc;

use iroh::endpoint::VarInt;

use crate::{Peer, PeerGroup};

impl PeerGroup {
    pub fn terminate<E, R>(peer_group: &Self, error_code: E, reason: &R)
    where
        E: Into<VarInt> + Send,
        R: AsRef<[u8]> + Send,
    {
        let error_code = error_code.into();
        let reason = reason.as_ref();

        let peers: Vec<Arc<Peer>> = {
            peer_group
                .peers
                .read()
                .iter()
                .map(|(_, peer)| peer.clone())
                .collect()
        };

        for peer in peers {
            Peer::terminate(&peer, error_code, &reason);
        }

        peer_group.state.write().terminated = true;
    }
}
