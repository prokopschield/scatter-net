use std::ops::Deref;

use ps_rwt::RWT;

use crate::{ScatterNet, ScatterNetInnerReadonly, ScatterNetInnerWritable};

impl Deref for ScatterNet {
    type Target = RWT<ScatterNetInnerReadonly, ScatterNetInnerWritable>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
