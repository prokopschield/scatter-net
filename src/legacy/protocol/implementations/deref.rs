use std::ops::Deref;

use crate::{ScatterNet, ScatterNetProtocol};

impl Deref for ScatterNetProtocol {
    type Target = ScatterNet;

    fn deref(&self) -> &Self::Target {
        &self.net
    }
}
