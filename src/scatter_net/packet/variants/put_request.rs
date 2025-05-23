use bytes::Bytes;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PutRequest {
    /// Data to be stored, either a raw blob, or an encrypted `DataChunk`
    pub data: Bytes,
}
