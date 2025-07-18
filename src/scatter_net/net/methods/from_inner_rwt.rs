use ps_rwt::RWT;

use crate::{ScatterNet, ScatterNetInnerReadonly, ScatterNetInnerWritable};

impl ScatterNet {
    #[must_use]
    pub const fn from_inner_rwt(
        inner: RWT<ScatterNetInnerReadonly, ScatterNetInnerWritable>,
    ) -> Self {
        Self { inner }
    }
}
