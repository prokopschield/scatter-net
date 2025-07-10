use iroh::NodeId;

use crate::ScatterNet;

impl ScatterNet {
    #[must_use]
    pub fn get_node_id(&self) -> NodeId {
        self.endpoint.node_id()
    }
}
