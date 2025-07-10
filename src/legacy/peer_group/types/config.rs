use iroh::NodeId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct PeerGroupConfig {
    pub members: Vec<NodeId>,
    #[serde(default = "default_name")]
    pub name: String,
    pub open: bool,
    pub rtt_cap_ms: u64,
}

fn default_name() -> String {
    "unnamed_peer_group".to_string()
}
