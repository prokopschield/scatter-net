mod implementations;
mod inner;
mod methods;

pub use implementations::*;
pub use inner::*;
use ps_rwt::RWT;

#[derive(Clone, Debug)]
pub struct PeerGroup {
    inner: RWT<PeerGroupInnerReadonly, PeerGroupInnerWritable>,
}
