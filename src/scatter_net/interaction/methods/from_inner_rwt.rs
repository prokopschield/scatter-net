use ps_rwt::RWT;

use crate::{Interaction, InteractionInnerReadonly, InteractionInnerWritable};

impl Interaction {
    #[must_use]
    pub const fn from_inner_rwt(
        inner: RWT<InteractionInnerReadonly, InteractionInnerWritable>,
    ) -> Self {
        Self { inner }
    }
}
