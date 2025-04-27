mod methods;
mod variants;

pub use methods::*;
pub use variants::{FetchRequest, FetchResponse, PutRequest, PutResponse};

use serde::{Deserialize, Serialize};

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
