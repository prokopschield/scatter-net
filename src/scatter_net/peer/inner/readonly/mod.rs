use iroh::EndpointId;

use crate::ScatterNet;

#[derive(Clone, Debug)]
pub struct PeerInnerReadonly {
    pub net: ScatterNet,
    pub node_id: EndpointId,
}
