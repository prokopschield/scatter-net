use crate::PeerGroupAsyncStoreError;

#[derive(thiserror::Error, Debug)]
pub enum PutError {
    #[error("Failed to store chunk locally or with any Peer.")]
    Failure,
    #[error("Failed to store chunk in PeerGroup: {0}")]
    PeerGroup(#[from] PeerGroupAsyncStoreError),
}
