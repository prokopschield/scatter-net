use iroh::NodeId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PeerState {
    pub node_id: NodeId,
    #[serde(default)]
    pub terminated: bool,
}
