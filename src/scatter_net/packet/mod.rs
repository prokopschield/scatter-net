mod variants;

use rkyv::{Archive, Deserialize, Serialize};
use variants::{FetchRequest, FetchResponse, PutRequest, PutResponse};

#[derive(
    Archive, Serialize, Deserialize, Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
pub enum Packet {
    #[default]
    Empty,

    Ping,
    Pong,

    FetchRequest(FetchRequest),
    FetchResponse(FetchResponse),

    PutRequest(PutRequest),
    PutResponse(PutResponse),
}
