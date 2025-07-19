use std::ops::Deref;

use ps_rwt::RWT;

use crate::{Peer, PeerInnerReadonly, PeerInnerWritable};

impl Deref for Peer {
    type Target = RWT<PeerInnerReadonly, PeerInnerWritable>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
