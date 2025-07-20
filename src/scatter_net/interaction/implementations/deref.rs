use std::ops::Deref;

use ps_rwt::RWT;

use crate::{Interaction, InteractionInnerReadonly, InteractionInnerWritable};

impl Deref for Interaction {
    type Target = RWT<InteractionInnerReadonly, InteractionInnerWritable>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
