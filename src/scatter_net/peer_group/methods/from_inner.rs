use ps_rwt::RWT;

use crate::{PeerGroup, PeerGroupInnerReadonly, PeerGroupInnerWritable};

impl PeerGroup {
    #[must_use]
    pub fn from_inner(readonly: PeerGroupInnerReadonly, writable: PeerGroupInnerWritable) -> Self {
        Self::from_inner_rwt(RWT::new(readonly, writable))
    }
}
