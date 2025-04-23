use iroh::NodeId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PeerGroupConfig {
    pub members: Vec<NodeId>,
    pub name: String,
    pub open: bool,
    pub rtt_cap_ms: u64,
}
