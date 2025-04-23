use std::sync::Arc;

use iroh::endpoint::VarInt;

use crate::{Peer, PeerGroup, ScatterNet};

impl ScatterNet {
    pub fn terminate<E, R>(net: &Self, error_code: E, reason: &R)
    where
        E: Into<VarInt> + Send,
        R: AsRef<[u8]> + Send,
    {
        let error_code = error_code.into();
        let reason = reason.as_ref();

        let peer_groups = net.peer_groups.read().clone();

        for peer_group in peer_groups {
            PeerGroup::terminate(&peer_group, error_code, &reason);
        }

        let peers: Vec<Arc<Peer>> = net
            .peers
            .read()
            .iter()
            .map(|(_, peer)| peer.clone())
            .collect();

        for peer in peers {
            Peer::terminate(&peer, error_code, &reason);
        }
    }
}
