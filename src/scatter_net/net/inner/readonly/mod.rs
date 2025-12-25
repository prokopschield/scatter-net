use iroh::{Endpoint, EndpointId};
use ps_datalake::lake::DataLake;

use crate::NetConfig;

#[derive(Debug)]
pub struct ScatterNetInnerReadonly {
    pub config: NetConfig,
    pub endpoint: Endpoint,
    pub lake: DataLake<'static>,
    pub node_id: EndpointId,
}
