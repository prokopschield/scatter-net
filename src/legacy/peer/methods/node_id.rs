use iroh::EndpointId;

use crate::Peer;

impl Peer {
    #[must_use]
    pub fn node_id(&self) -> EndpointId {
        self.node_id
    }
}
