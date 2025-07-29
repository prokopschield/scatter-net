mod builder;
mod implementations;
mod inner;
mod methods;
mod state;

pub use builder::*;
pub use implementations::*;
pub use inner::*;
use ps_rwt::RWT;
pub use state::*;

#[derive(Clone, Debug)]
pub struct Peer {
    inner: RWT<PeerInnerReadonly, PeerInnerWritable>,
}
