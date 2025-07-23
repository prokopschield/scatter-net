use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FetchRequest {
    /// Hash of the requested encrypted blob
    pub hash: String,
}
