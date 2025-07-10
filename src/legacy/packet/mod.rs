mod methods;
mod variants;

pub use methods::*;
use serde::{Deserialize, Serialize};
pub use variants::{FetchRequest, FetchResponse, PutRequest, PutResponse};

#[derive(Serialize, Deserialize, Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Packet {
    #[default]
    Empty,

    /// This is a general, unspecified error.
    Error,

    Ping,
    Pong,

    FetchRequest(FetchRequest),
    FetchResponse(FetchResponse),

    PutRequest(PutRequest),
    PutResponse(PutResponse),
}
