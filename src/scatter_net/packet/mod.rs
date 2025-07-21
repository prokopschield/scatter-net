mod methods;

use serde::{Deserialize, Serialize};

use crate::{FetchRequest, FetchResponse, PutRequest, PutResponse};

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Packet {
    #[default]
    Empty,

    /// This represents a generic error.
    Error(String),

    Ping,
    Pong,

    FetchRequest(FetchRequest),
    FetchResponse(FetchResponse),

    PutRequest(PutRequest),
    PutResponse(PutResponse),
}
