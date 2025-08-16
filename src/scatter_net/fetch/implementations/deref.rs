use std::ops::Deref;

use ps_rwt::RWT;

use crate::{Fetch, FetchInnerReadonly, FetchInnerWritable};

impl Deref for Fetch {
    type Target = RWT<FetchInnerReadonly, FetchInnerWritable>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
