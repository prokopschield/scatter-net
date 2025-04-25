use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum FetchResponse {
    /// Requestee couldn't fulfill this request due to an error.
    Error,

    /// Requestee did not have this DataChunk
    NotFound,

    /// Successfully retreived encrypted DataChunk.
    Success(Vec<u8>),

    /// Suggests a node to talk to.
    Suggest(Vec<u8>),
}
