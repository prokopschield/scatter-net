use iroh::NodeId;
use serde::{Deserialize, Serialize};

use super::PeerUsage;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PeerState {
    pub node_id: NodeId,
    #[serde(default)]
    pub terminated: bool,
    pub usage: PeerUsage,
}
