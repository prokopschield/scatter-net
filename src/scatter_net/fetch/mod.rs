mod error;
mod implementations;
mod inner;
mod methods;

pub use error::FetchError;
pub use inner::*;
use ps_rwt::RWT;

#[derive(Clone, Debug)]
pub struct Fetch {
    inner: RWT<FetchInnerReadonly, FetchInnerWritable>,
}
