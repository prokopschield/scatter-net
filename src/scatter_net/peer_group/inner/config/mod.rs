use iroh::EndpointId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PeerGroupConfig {
    #[serde(default = "default_name")]
    pub name: String,

    /// `Peer`s in this `PeerGroup`
    pub members: Vec<EndpointId>,

    /// Add `Peer`s to this `PeerGroup` automatically?
    pub open: bool,

    /// `Peer`s will be added into this `PeerGroup` if the round-trip ping time
    /// to the `Peer` falls under `rtt_cap_ms`.
    ///
    /// `Peer`s will not be added if this `PeerGroup` isn't `open`.
    pub rtt_cap_ms: u64,
}

fn default_name() -> String {
    "unnamed_peer_group".to_string()
}
