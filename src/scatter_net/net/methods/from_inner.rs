use ps_rwt::RWT;

use crate::{ScatterNet, ScatterNetInnerReadonly, ScatterNetInnerWritable};

impl ScatterNet {
    #[must_use]
    pub fn from_inner(
        readonly: ScatterNetInnerReadonly,
        writable: ScatterNetInnerWritable,
    ) -> Self {
        Self::from_inner_rwt(RWT::new(readonly, writable))
    }
}
