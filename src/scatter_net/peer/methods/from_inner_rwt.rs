use ps_rwt::RWT;

use crate::{Peer, PeerInnerReadonly, PeerInnerWritable};

impl Peer {
    #[must_use]
    pub const fn from_inner_rwt(inner: RWT<PeerInnerReadonly, PeerInnerWritable>) -> Self {
        Self { inner }
    }
}
