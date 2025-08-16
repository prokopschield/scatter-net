use ps_rwt::RWT;

use crate::{Fetch, FetchInnerReadonly, FetchInnerWritable};

impl Fetch {
    #[must_use]
    pub const fn from_inner_rwt(inner: RWT<FetchInnerReadonly, FetchInnerWritable>) -> Self {
        Self { inner }
    }
}
