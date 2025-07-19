mod implementations;
mod inner;
mod methods;

pub use inner::*;
use ps_rwt::RWT;

#[derive(Clone, Debug)]
pub struct Peer {
    inner: RWT<PeerInnerReadonly, PeerInnerWritable>,
}
