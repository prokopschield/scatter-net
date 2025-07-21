mod implementations;
mod inner;
mod methods;

pub use inner::*;
pub use methods::*;
use ps_rwt::RWT;

#[derive(Clone, Debug)]
pub struct Interaction {
    inner: RWT<InteractionInnerReadonly, InteractionInnerWritable>,
}
