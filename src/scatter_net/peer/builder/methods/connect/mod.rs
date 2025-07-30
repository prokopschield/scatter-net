mod error;
mod implementations;


pub use error::*;
use iroh::endpoint::Connecting;
use ps_promise::Promise;

use crate::PeerBuilder;

#[derive(Clone, Debug)]
pub enum PeerBuilderConnect {
    Initial {
        builder: PeerBuilder
    },
    Connecting {
        builder: PeerBuilder,
        connecting: Promise<Connecting>
    },
    Placeholder,
}