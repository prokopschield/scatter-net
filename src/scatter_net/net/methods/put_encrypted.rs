use std::{future::Future, sync::Arc};

use bytes::Bytes;
use n0_future::FutureExt;
use ps_buffer::{SharedBuffer, ToSharedBuffer};
use ps_datachunk::OwnedDataChunk;
use ps_hash::Hash;
use ps_hkey::Hkey;
use ps_promise::Promise;

use crate::{Peer, PeerGroup, PeerPutBlobError, PutResponse, ScatterNet};

impl ScatterNet {
    /// `chunk` should be backed by a smart pointer that is cheap to clone, e.g. `Arc<[u8]>` or `SharedBuffer`
    #[must_use = "Futures don't do anything unless awaited or polled."]
    pub fn put_encrypted<D: AsRef<[u8]>>(
        self: Arc<Self>,
        data: D,
    ) -> Result<ScatterNetPutEncrypted, ScatterNetPutEncryptedError> {
        let buffer = data.as_ref().to_shared_buffer()?;
        let hash = Arc::new(ps_hash::hash(&buffer)?);

        Ok(ScatterNetPutEncrypted::Initial {
            buffer,
            hash,
            net: self,
        })
    }
}

// TODO #[derive(Debug)]
pub enum ScatterNetPutEncrypted {
    Initial {
        buffer: SharedBuffer,
        hash: Arc<Hash>,
        net: Arc<ScatterNet>,
    },
    Sending {
        buffer: SharedBuffer,
        hash: Arc<Hash>,
        hkey: Option<Hkey>,
        targets: Vec<Target>,
    },
    Success {
        hkey: Hkey,
    },
    Placeholder,
}

#[derive(Debug)]
pub struct Target {
    hkey: Option<Hkey>,
    peer: Option<Arc<Peer>>,
    peer_group: Arc<PeerGroup>,
    promise: Option<Promise<Hkey, PutResponseInternalError>>,
}

impl Future for ScatterNetPutEncrypted {
    type Output = Result<Hkey, ScatterNetPutEncryptedError>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        use std::task::Poll::{Pending, Ready};

        use ScatterNetPutEncryptedError::{NoPeers, PlaceholderState};

        let this = self.get_mut();

        loop {
            match std::mem::replace(this, Self::Placeholder) {
                Self::Initial { buffer, hash, net } => {
                    let hkey = net
                        .lake
                        .put_encrypted_chunk(&OwnedDataChunk::from_data_and_hash(
                            buffer.clone(),
                            hash.clone(),
                        ))
                        .ok();

                    *this = Self::Sending {
                        buffer,
                        hash,
                        hkey,
                        targets: net
                            .peer_groups
                            .read()
                            .iter()
                            .map(|peer_group| Target {
                                hkey: None,
                                peer: None,
                                peer_group: peer_group.clone(),
                                promise: None,
                            })
                            .collect(),
                    };
                }

                Self::Sending {
                    buffer,
                    hash,
                    hkey,
                    mut targets,
                } => {
                    let mut need_iteration = false;

                    for target in &mut targets {
                        if let Some(promise) = &mut target.promise {
                            if let Ready(result) = promise.poll(cx) {
                                if let Ok(hkey) = result {
                                    target.hkey = Some(hkey);
                                } else {
                                    target.peer = None;
                                }

                                target.promise = None;
                            }
                        };

                        if target.hkey.is_some() {
                            continue;
                        }

                        target.peer = target.peer_group.get_peer_by_hash(&hash);

                        if let Some(peer) = &target.peer {
                            target.promise = Some(
                                Promise::new({
                                    let bytes = Bytes::from_owner(buffer.clone());
                                    let peer = peer.clone();

                                    peer.put_blob(bytes)
                                })
                                .then(|response| match response {
                                    PutResponse::Success(hkey) => Ok(Hkey::parse(hkey.as_bytes())),
                                    PutResponse::Failure => Err(PutResponseInternalError::Failure),
                                    PutResponse::LimitExceeded => {
                                        Err(PutResponseInternalError::LimitExceeded)
                                    }
                                }),
                            );

                            need_iteration = true;
                        }
                    }

                    if targets.iter().all(|target| target.promise.is_none()) {
                        let hkey = hkey.or_else(|| {
                            targets
                                .iter()
                                .find(|target| target.hkey.is_some())
                                .and_then(|target| target.hkey.clone())
                        });

                        if let Some(hkey) = hkey {
                            *this = Self::Success { hkey };
                        } else {
                            *this = Self::Sending {
                                buffer,
                                hash,
                                hkey,
                                targets,
                            };

                            return Ready(Err(NoPeers));
                        }
                    } else {
                        *this = Self::Sending {
                            buffer,
                            hash,
                            hkey,
                            targets,
                        };
                    }

                    if !need_iteration {
                        return Pending;
                    }
                }

                Self::Success { hkey } => {
                    *this = Self::Success { hkey: hkey.clone() };

                    return Ready(Ok(hkey));
                }

                Self::Placeholder => return Ready(Err(PlaceholderState)),
            }
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ScatterNetPutEncryptedError {
    #[error("Buffer error")]
    Buffer(#[from] ps_buffer::BufferError),
    #[error("Hash error: {0}")]
    Hash(#[from] ps_hash::HashError),
    #[error("No peers to push to")]
    NoPeers,
    #[error("Future is in an invalid state, it probably paniced.")]
    PlaceholderState,
}

#[derive(thiserror::Error, Debug)]
enum PutResponseInternalError {
    #[error("Failed")]
    Failure,
    #[error("LimitExceeded")]
    LimitExceeded,
    #[error(transparent)]
    PeerPutBlobError(#[from] PeerPutBlobError),
}
