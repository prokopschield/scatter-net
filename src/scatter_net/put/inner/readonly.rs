use std::sync::Arc;

use ps_hash::Hash;

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PutInnerReadonly {
    /// Stored in an [`Arc`] for convenience: [`Arc<Hash>`] is commonly used elsewhere
    pub hash: Arc<Hash>,
}
