mod methods;

use bytes::Bytes;
use iroh::NodeId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum FetchResponse {
    /// Requestee couldn't fulfill this request due to an error.
    Error(String),

    /// Requestee did not have this `DataChunk`
    NotFound,

    /// Successfully retreived encrypted `DataChunk`.
    Success(Bytes),

    /// Suggests a node to talk to.
    Suggest(NodeId),
}
