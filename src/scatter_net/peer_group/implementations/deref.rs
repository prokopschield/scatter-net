use std::ops::Deref;

use ps_rwt::RWT;

use crate::{PeerGroup, PeerGroupInnerReadonly, PeerGroupInnerWritable};

impl Deref for PeerGroup {
    type Target = RWT<PeerGroupInnerReadonly, PeerGroupInnerWritable>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
