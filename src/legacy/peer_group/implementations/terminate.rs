use std::sync::Arc;

use iroh::endpoint::VarInt;

use crate::{Peer, PeerGroup, Terminate};

impl<E, R> Terminate<E, R> for PeerGroup
where
    E: Into<VarInt> + Send,
    R: AsRef<[u8]> + Send,
{
    fn terminate(self: &Arc<Self>, error_code: E, reason: &R) {
        let error_code = error_code.into();
        let reason = reason.as_ref();

        let peers: Vec<Arc<Peer>> = {
            self.peers
                .read()
                .iter()
                .map(|(_, peer)| peer.clone())
                .collect()
        };

        for peer in peers {
            peer.terminate(error_code, &reason);
        }
    }
}
