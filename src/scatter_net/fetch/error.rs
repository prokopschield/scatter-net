use std::sync::Arc;

use ps_hash::Hash;

#[derive(thiserror::Error, Debug)]
pub enum FetchError {
    #[error("Failed to fetch {0}: tried every Peer and failed.")]
    OptionsExhausted(Arc<Hash>),
}
