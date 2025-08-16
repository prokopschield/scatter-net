use ps_rwt::RWT;

use crate::{Fetch, FetchInnerReadonly, FetchInnerWritable};

impl Fetch {
    #[must_use]
    pub fn from_inner(readonly: FetchInnerReadonly, writable: FetchInnerWritable) -> Self {
        Self::from_inner_rwt(RWT::new(readonly, writable))
    }
}
