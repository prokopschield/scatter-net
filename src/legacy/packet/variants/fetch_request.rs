use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FetchRequest {
    /// Hash of the requested encrypted blob
    pub hash: String,

    /// Request level. 0 for local, 7 for interplanetary. 1..6 for everything inbetween.
    pub level: u8,

    /// Should requestee attempt to recursively locate the blob?
    pub recursive: bool,
}
