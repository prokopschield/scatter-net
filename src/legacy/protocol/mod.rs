mod implementations;
mod methods;

use std::sync::Arc;

use super::ScatterNet;

#[derive(Clone, Debug)]
pub struct ScatterNetProtocol {
    net: Arc<ScatterNet>,
}
