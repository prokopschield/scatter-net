use serde::{Deserialize, Serialize};

use crate::PeerState;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct NetState {
    pub peers: Vec<PeerState>,
    pub terminated: bool,
}
