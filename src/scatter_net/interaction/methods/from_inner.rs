use ps_rwt::RWT;

use crate::{Interaction, InteractionInnerReadonly, InteractionInnerWritable};

impl Interaction {
    #[must_use]
    pub fn from_inner(
        readonly: InteractionInnerReadonly,
        writable: InteractionInnerWritable,
    ) -> Self {
        Self::from_inner_rwt(RWT::new(readonly, writable))
    }
}
