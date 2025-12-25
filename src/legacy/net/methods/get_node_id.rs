use iroh::EndpointId;

use crate::ScatterNet;

impl ScatterNet {
    #[must_use]
    pub fn get_node_id(&self) -> EndpointId {
        self.endpoint.id()
    }
}
