use std::ops::Deref;

use ps_rwt::RWT;

use crate::{Put, PutInnerReadonly, PutInnerWritable};

impl Deref for Put {
    type Target = RWT<PutInnerReadonly, PutInnerWritable>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
