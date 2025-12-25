use iroh::EndpointId;
use serde::{Deserialize, Serialize};

use crate::PeerUsage;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PeerState {
    pub node_id: EndpointId,
    #[serde(default)]
    pub usage: PeerUsage,
    #[serde(default)]
    pub terminated: bool,
}
