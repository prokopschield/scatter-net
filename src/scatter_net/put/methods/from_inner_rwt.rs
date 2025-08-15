use ps_rwt::RWT;

use crate::{Put, PutInnerReadonly, PutInnerWritable};

impl Put {
    #[must_use]
    pub const fn from_inner_rwt(inner: RWT<PutInnerReadonly, PutInnerWritable>) -> Self {
        Self { inner }
    }
}
