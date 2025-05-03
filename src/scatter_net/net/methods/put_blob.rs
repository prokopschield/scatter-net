use std::{future::Future, pin::Pin, sync::Arc};

use anyhow::Result;
use bytes::Bytes;
use parking_lot::RwLock;
use ps_cypher::extract_encrypted;
use ps_hash::Hash;
use ps_hkey::{Hkey, LongHkeyExpanded};

use crate::{Peer, PeerGroup, PeerPutBlob, ScatterNet};

impl ScatterNet {
    pub fn put_blob(
        self: &Arc<Self>,
        blob: Bytes,
    ) -> Result<ScatterNetPutBlob, ScatterNetPutBlobError> {
        let hash = Hash::hash(&blob)?;

        if let Some(from_cache) = self.put_cache.read().get(&hash) {
            return Ok(from_cache.clone());
        }

        let hkey = self.lake.put_blob(&blob).ok();

        let future = ScatterNetPutBlob::new(blob, hash, hkey, self.clone());

        self.put_cache.write().insert(hash, future.clone());

        future.background();

        Ok(future)
    }
}

pub struct ScatterNetPutBlobInner {
    pub blob: RwLock<Option<Bytes>>,
    pub hash: Hash,
    pub hkey: RwLock<Option<Hkey>>,
    pub net: Arc<ScatterNet>,
    pub state: RwLock<State>,
}

pub struct Part {
    pub future: Pin<Box<ScatterNetPutBlob>>,
}

#[derive(Debug)]
pub struct Put {
    pub future: Option<Pin<Box<PeerPutBlob>>>,
    pub peer: Option<Arc<Peer>>,
    pub peer_group: Arc<PeerGroup>,
}

pub type LongHkeyResult = Result<LongHkeyExpanded, ScatterNetPutBlobError>;
pub type LongHkeyFuture = dyn Future<Output = LongHkeyResult> + Send + Sync;

pub enum State {
    Puts(Vec<Put>),
    Split(Pin<Box<LongHkeyFuture>>),
}

#[derive(Clone)]
pub struct ScatterNetPutBlob {
    inner: Arc<ScatterNetPutBlobInner>,
}

impl ScatterNetPutBlob {
    /// Executes this [`Future`] in the background via [`crate::spawn_and_forget`].
    pub fn background(&self) {
        let future = self.clone();

        crate::spawn_and_forget(async move { Ok(future.await?) });
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
    pub fn new(blob: Bytes, hash: Hash, hkey: Option<Hkey>, net: Arc<ScatterNet>) -> Self {
        // chunk + deflate + poly1305 + RS(255,231)
        // 4096 B + 5 B + 16 B + 496 B = 4613 B
        if blob.len() > 4613 {
            return Self::new_split(blob, hash, hkey, net);
        }

        let codeword = match extract_encrypted(&blob) {
            Err(_) => return Self::new_split(blob, hash, hkey, net),
            Ok(codeword) => codeword,
        };

        if *codeword.codeword == *blob {
            return Self::new_put(blob, hash, hkey, net);
        };

        // store corrected; try_into_buffer() is infallible here
        if let Ok(buffer) = codeword.codeword.try_into_buffer() {
            Self::new_put(Bytes::from_owner(buffer), hash, hkey.clone(), net.clone()).background();
        };

        // store actual blob with which put was called
        Self::new_split(blob, hash, hkey, net)
    }

    pub fn new_put(blob: Bytes, hash: Hash, hkey: Option<Hkey>, net: Arc<ScatterNet>) -> Self {
        let puts: Vec<Put> = net
            .peer_groups
            .read()
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
            hkey: RwLock::new(hkey),
            net,
            state: RwLock::new(State::Puts(puts)),
        };

        Self {
            inner: Arc::new(inner),
        }
    }

    pub fn new_split(blob: Bytes, hash: Hash, hkey: Option<Hkey>, net: Arc<ScatterNet>) -> Self {
        let net_clone = net.clone();

        let future = async move {
            LongHkeyExpanded::from_blob_async(
                &|data: &[u8]| {
                    let net = net_clone.clone();
                    let bytes = Bytes::copy_from_slice(data);
                    async move { net.put_blob(bytes)?.await }
                },
                &blob,
            )
            .await
        };

        let inner = ScatterNetPutBlobInner {
            blob: RwLock::new(None),
            hash,
            hkey: RwLock::new(hkey),
            net,
            state: RwLock::new(State::Split(Box::pin(future))),
        };

        Self {
            inner: Arc::new(inner),
        }
    }
}

impl Future for ScatterNetPutBlob {
    type Output = Result<Hkey, ScatterNetPutBlobError>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let _ = cx;
        todo!()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ScatterNetPutBlobError {
    #[error(transparent)]
    Hash(#[from] ps_hash::HashError),
    #[error(transparent)]
    Hkey(#[from] ps_hkey::PsHkeyError),
}
