use ps_rwt::RWT;

use crate::{Peer, PeerInnerReadonly, PeerInnerWritable};

impl Peer {
    #[must_use]
    pub fn from_inner(readonly: PeerInnerReadonly, writable: PeerInnerWritable) -> Self {
        Self::from_inner_rwt(RWT::new(readonly, writable))
    }
}
