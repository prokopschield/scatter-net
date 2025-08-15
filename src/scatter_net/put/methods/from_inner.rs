use ps_rwt::RWT;

use crate::{Put, PutInnerReadonly, PutInnerWritable};

impl Put {
    #[must_use]
    pub fn from_inner(readonly: PutInnerReadonly, writable: PutInnerWritable) -> Self {
        Self::from_inner_rwt(RWT::new(readonly, writable))
    }
}
