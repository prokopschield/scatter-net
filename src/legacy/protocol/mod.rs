mod implementations;
mod methods;

use std::sync::Arc;

use crate::ScatterNet;

#[derive(Clone, Debug)]
pub struct ScatterNetProtocol {
    net: Arc<ScatterNet>,
}
