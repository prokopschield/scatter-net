use anyhow::Result;
use iroh::NodeId;

use crate::Peer;

impl Peer {
    pub fn node_id(&self) -> Result<NodeId> {
        self.connection.read().remote_node_id()
    }
}
