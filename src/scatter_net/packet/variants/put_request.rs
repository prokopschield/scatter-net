use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PutRequest {
    /// Data to be stored, either a raw blob, or an encrypted DataChunk
    data: Vec<u8>,

    /// Is the data an encrypted DataChunk?
    encrypted: bool,
}
