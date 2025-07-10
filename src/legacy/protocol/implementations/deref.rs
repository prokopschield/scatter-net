use std::{ops::Deref, sync::Arc};

use crate::{ScatterNet, ScatterNetProtocol};

impl Deref for ScatterNetProtocol {
    type Target = Arc<ScatterNet>;

    fn deref(&self) -> &Self::Target {
        &self.net
    }
}
