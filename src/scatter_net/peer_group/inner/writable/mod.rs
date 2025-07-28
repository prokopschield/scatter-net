use std::collections::HashMap;

use iroh::NodeId;

use crate::{Peer, PeerGroupConfig};

#[derive(Clone, Debug)]
pub struct PeerGroupInnerWritable {
    pub config: PeerGroupConfig,
    pub peers: HashMap<NodeId, Peer>,
}
