mod error;
mod implementations;
mod inner;
mod methods;

pub use error::PutError;
pub use inner::*;
use ps_rwt::RWT;

#[derive(Clone, Debug)]
pub struct Put {
    inner: RWT<PutInnerReadonly, PutInnerWritable>,
}
