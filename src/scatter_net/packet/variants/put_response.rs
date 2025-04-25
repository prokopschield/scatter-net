use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum PutResponse {
    /// Blob has successfully been stored.
    Success(String),

    /// Failed to store the blob.
    Failure,

    /// Refused to store the blob due to a rate limit being exceeded.
    LimitExceeded,
}
