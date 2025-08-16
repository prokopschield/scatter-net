use std::sync::Arc;

use ps_hash::Hash;

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FetchInnerReadonly {
    pub hash: Arc<Hash>,
}
