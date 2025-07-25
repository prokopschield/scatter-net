use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::Poll::{Pending, Ready},
};

use bytes::Bytes;
use n0_future::FutureExt;
use parking_lot::RwLock;
use ps_buffer::ToSharedBuffer;
use ps_cypher::extract_encrypted;
use ps_datachunk::{BorrowedDataChunk, DataChunk};
use ps_hash::Hash;
use ps_hkey::{Hkey, LongHkeyExpanded};
use ps_promise::{Promise, PromiseRejection};

use crate::{AsyncStoreError, Peer, PeerGroup, PeerPutBlobError, PutResponse, ScatterNet};

use super::{ScatterNetPutEncrypted, ScatterNetPutRaw};

impl ScatterNet {
    pub fn put_blob(&self, blob: Bytes) -> Result {
        let hash = Arc::new(Hash::hash(&blob)?);

        if let Some(from_cache) = self.read().put_cache.get(&hash) {
            return Ok(from_cache.clone());
        }

        let future = ScatterNetPutBlob::new(blob, hash.clone(), self.clone())?;

        self.write().put_cache.insert(hash, future.clone());

        future.background();

        Ok(future)
    }
}

#[derive(Debug)]
pub struct ScatterNetPutBlobInner {
    pub blob: RwLock<Option<Bytes>>,
    pub hash: Arc<Hash>,
    pub hkey: RwLock<Option<Hkey>>,
    pub net: ScatterNet,
    pub state: RwLock<State>,
}

pub struct Part {
    pub future: Pin<Box<ScatterNetPutBlob>>,
}

#[derive(Debug)]
pub struct Put {
    pub future: Option<Promise<PutResponse, PeerPutBlobError>>,
    pub peer: Option<Peer>,
    pub peer_group: Arc<PeerGroup>,
}

#[derive(Debug)]
pub enum State {
    PutEncrypted(ScatterNetPutEncrypted),
    PutRaw(ScatterNetPutRaw),
    Puts(Vec<Put>),
    Split(Promise<Hkey, ScatterNetPutBlobError>),
}

#[derive(Clone, Debug)]
pub struct ScatterNetPutBlob {
    inner: Arc<ScatterNetPutBlobInner>,
}

impl ScatterNetPutBlob {
    /// Executes this [`Future`] in the background via [`crate::spawn_and_forget`].
    pub fn background(&self) {
        let future = self.clone();

        crate::spawn_and_forget(async move { Ok(future.await?) });
    }

    #[inline]
    pub const fn early_return(self) -> ScatterNetPutBlobEarlyReturn {
        ScatterNetPutBlobEarlyReturn { future: self }
    }

    #[must_use]
    pub fn get_blob(&self) -> Option<Bytes> {
        self.inner.blob.read().clone()
    }

    #[must_use]
    pub fn get_hash(&self) -> &Hash {
        &self.inner.hash
    }

    #[must_use]
    pub fn get_hkey(&self) -> Option<Hkey> {
        self.inner.hkey.read().clone()
    }

    #[inline]
    pub fn new(blob: Bytes, hash: Arc<Hash>, net: ScatterNet) -> Result {
        // chunk + deflate + poly1305 + RS(255,231)
        // 4096 B + 5 B + 16 B + 496 B = 4613 B
        if blob.len() > 4613 {
            return Self::new_split(blob, hash, net);
        }

        let codeword = match extract_encrypted(&blob) {
            Err(_) => {
                return Self::new_put_raw(blob, hash, net);
            }
            Ok(codeword) => codeword,
        };

        if *codeword.codeword == *blob {
            return Self::new_put_encrypted(blob, hash, net);
        }

        // store corrected; try_into_buffer() is infallible here
        if let Ok(buffer) = codeword.codeword.try_into_buffer() {
            Self::new_put(Bytes::from_owner(buffer), hash.clone(), net.clone())?.background();
        }

        // store actual blob with which put was called
        Self::new_put_raw(blob, hash, net)
    }

    pub fn new_put_encrypted(blob: Bytes, hash: Arc<Hash>, net: ScatterNet) -> Result {
        let chunk = BorrowedDataChunk::from_parts(&blob, hash.clone());
        let hkey = net
            .lake
            .put_encrypted_chunk(&chunk)
            .unwrap_or_else(|_| hash.clone().into());

        let put = net.clone().put_encrypted(&blob)?;

        let inner = ScatterNetPutBlobInner {
            blob: RwLock::new(Some(blob)),
            hash,
            hkey: RwLock::new(Some(hkey)),
            net,
            state: RwLock::new(State::PutEncrypted(put)),
        };

        Ok(Self {
            inner: Arc::new(inner),
        })
    }

    pub fn new_put_raw(blob: Bytes, hash: Arc<Hash>, net: ScatterNet) -> Result {
        let chunk = BorrowedDataChunk::from_parts(&blob, hash.clone());
        let hkey = net
            .lake
            .put_encrypted_chunk(&chunk)
            .unwrap_or_else(|_| hash.clone().into());

        let put = net.clone().put_raw(&blob)?;

        let inner = ScatterNetPutBlobInner {
            blob: RwLock::new(Some(blob)),
            hash,
            hkey: RwLock::new(Some(hkey)),
            net,
            state: RwLock::new(State::PutRaw(put)),
        };

        Ok(Self {
            inner: Arc::new(inner),
        })
    }

    pub fn new_put(blob: Bytes, hash: Arc<Hash>, net: ScatterNet) -> Result {
        let chunk = BorrowedDataChunk::from_parts(&blob, hash.clone());
        let hkey = net
            .lake
            .put_encrypted_chunk(&chunk)
            .unwrap_or_else(|_| hash.clone().into());

        let puts: Vec<Put> = net
            .read()
            .peer_groups
            .iter()
            .map(|group| Put {
                future: None,
                peer: None,
                peer_group: group.clone(),
            })
            .collect();

        let inner = ScatterNetPutBlobInner {
            blob: RwLock::new(Some(blob)),
            hash,
            hkey: RwLock::new(Some(hkey)),
            net,
            state: RwLock::new(State::Puts(puts)),
        };

        Ok(Self {
            inner: Arc::new(inner),
        })
    }

    pub fn new_split(blob: Bytes, hash: Arc<Hash>, net: ScatterNet) -> Result {
        let hkey = net.lake.put_blob(&blob).ok();

        let net_clone = net.clone();

        let future = async move {
            let hkey = LongHkeyExpanded::from_blob_async::<_, ScatterNetPutBlobError, _, _>(
                &net_clone, &blob,
            )
            .await?;

            let hkey = hkey
                .shrink_async::<_, ScatterNetPutBlobError, _, _>(&net_clone)
                .await?;

            Ok(hkey)
        };

        let inner = ScatterNetPutBlobInner {
            blob: RwLock::new(None),
            hash,
            hkey: RwLock::new(hkey),
            net,
            state: RwLock::new(State::Split(Promise::new(future))),
        };

        Ok(Self {
            inner: Arc::new(inner),
        })
    }

    /// Performs a single iteration of the future, advancing the process of propagating the blob to all intended peers.
    pub fn run(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> Result<Option<Hkey>, ScatterNetPutBlobError> {
        use PutResponse::{Failure, LimitExceeded, Success};

        let mut guard = self.inner.state.write();

        if let State::Puts(puts) = &mut *guard {
            let mut pending = false;

            for put in puts {
                let redo = match &mut put.future {
                    None => true,
                    Some(promise) => match promise.poll(cx) {
                        Pending => {
                            pending = true;
                            false
                        }
                        Ready(Err(_) | Ok(Failure | LimitExceeded)) => true,
                        Ready(Ok(Success(hkey_str))) => {
                            let redo = hkey_str.as_bytes() != self.inner.hash.as_bytes();
                            *promise = Promise::Resolved(PutResponse::Success(hkey_str));
                            redo
                        }
                    },
                };

                if !redo {
                    continue;
                }

                pending = true;

                let Some(peer) = put.peer_group.get_peer_by_hash(self.get_hash()) else {
                    continue;
                };

                let bytes = self.inner.blob.read().clone();
                let bytes = if let Some(bytes) = bytes {
                    bytes
                } else {
                    let chunk = self.inner.net.lake.get_encrypted_chunk(self.get_hash())?;
                    let buffer = chunk.data_ref().to_shared_buffer()?;
                    let bytes = Bytes::from_owner(buffer);

                    *self.inner.blob.write() = Some(bytes.clone());

                    bytes
                };

                let future = peer.clone().put_blob(bytes);

                put.peer = Some(peer);
                put.future = Some(Promise::new(future));
            }

            if pending {
                Ok(None)
            } else {
                Ok(self
                    .get_hkey()
                    .or_else(|| Some(Hkey::Direct(self.inner.hash.clone()))))
            }
        } else if let State::Split(promise) = &mut *guard {
            let poll = promise.poll(cx);

            if let Ready(result) = poll {
                match result {
                    Ok(hkey) => {
                        *promise = Promise::Resolved(hkey.clone());
                        *self.inner.hkey.write() = Some(hkey.clone());

                        Ok(Some(hkey))
                    }
                    Err(err) => Err(err),
                }
            } else {
                Ok(None)
            }
        } else if let State::PutEncrypted(encrypted) = &mut *guard {
            match encrypted.poll(cx) {
                Pending => Ok(None),
                Ready(Ok(hkey)) => Ok(Some(hkey)),
                Ready(Err(err)) => Err(err)?,
            }
        } else if let State::PutRaw(raw) = &mut *guard {
            match raw.poll(cx) {
                Pending => Ok(None),
                Ready(Ok(hkey)) => Ok(Some(hkey)),
                Ready(Err(err)) => Err(err)?,
            }
        } else {
            unreachable!()
        }
    }
}

impl Future for ScatterNetPutBlob {
    type Output = Result<Hkey>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        match self.get_mut().run(cx) {
            Ok(None) => std::task::Poll::Pending,
            Ok(Some(hkey)) => std::task::Poll::Ready(Ok(hkey)),
            Err(err) => std::task::Poll::Ready(Err(err)),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ScatterNetPutBlobError {
    #[error(transparent)]
    AsyncStore(Box<AsyncStoreError>),
    #[error(transparent)]
    Buffer(#[from] ps_buffer::BufferError),
    #[error(transparent)]
    Hash(#[from] ps_hash::HashError),
    #[error(transparent)]
    Hkey(#[from] ps_hkey::PsHkeyError),
    #[error(transparent)]
    Lake(#[from] ps_datalake::error::PsDataLakeError),
    #[error("Promise consumed more than once")]
    PromiseAlreadyConsumed,
    #[error(transparent)]
    PutEncrypted(#[from] crate::ScatterNetPutEncryptedError),
    #[error(transparent)]
    PutRaw(#[from] crate::ScatterNetPutRawError),
}

type Result<T = ScatterNetPutBlob, E = ScatterNetPutBlobError> = std::result::Result<T, E>;

impl From<AsyncStoreError> for ScatterNetPutBlobError {
    fn from(value: AsyncStoreError) -> Self {
        Self::AsyncStore(Box::new(value))
    }
}

impl PromiseRejection for ScatterNetPutBlobError {
    fn already_consumed() -> Self {
        Self::PromiseAlreadyConsumed
    }
}

#[must_use = "This Future doesn't do anything unless polled or awaited."]
pub struct ScatterNetPutBlobEarlyReturn {
    future: ScatterNetPutBlob,
}

impl Future for ScatterNetPutBlobEarlyReturn {
    type Output = Result<Hkey>;

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if let Some(hkey) = self.future.get_hkey() {
            return Ready(Ok(hkey));
        }

        self.get_mut().future.poll(cx)
    }
}
