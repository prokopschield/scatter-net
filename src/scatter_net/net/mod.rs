mod config;
mod implementations;
mod inner;
mod methods;

pub use config::*;
pub use inner::*;
pub use methods::*;
use ps_rwt::RWT;

#[derive(Clone)]
pub struct ScatterNet {
    inner: RWT<ScatterNetInnerReadonly, ScatterNetInnerWritable>,
}
