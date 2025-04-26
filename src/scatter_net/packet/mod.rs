mod variants;

use serde::{Deserialize, Serialize};
use variants::{FetchRequest, FetchResponse, PutRequest, PutResponse};

#[derive(Serialize, Deserialize, Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
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
