mod implementations;
mod inner;
mod methods;

pub use inner::*;
use ps_rwt::RWT;

#[derive(Clone)]
pub struct ScatterNet {
    inner: RWT<ScatterNetInnerReadonly, ScatterNetInnerWritable>,
}
