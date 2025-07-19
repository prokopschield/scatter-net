mod implementations;
mod methods;

use crate::ScatterNet;

#[derive(Clone, Debug)]
pub struct ScatterNetProtocol {
    net: ScatterNet,
}
