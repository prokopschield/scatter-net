use std::{future::Future, pin::Pin, sync::Arc};

use anyhow::Result;
use bytes::Bytes;
use parking_lot::RwLock;
use ps_hash::Hash;
use ps_hkey::Hkey;

use crate::{Peer, PeerGroup, PeerPutBlob, ScatterNet};

impl ScatterNet {
    pub fn put_blob(
        net: &Arc<Self>,
        blob: Bytes,
    ) -> Result<ScatterNetPutBlob, ScatterNetPutBlobError> {
        let hash = Hash::hash(&blob)?;

        if let Some(from_cache) = net.put_cache.read().get(&hash) {
            return Ok(from_cache.clone());
        }

        let hkey = net.lake.put_blob(&blob).ok();

        let future = ScatterNetPutBlob::new(blob, hash, hkey, net.clone());

        net.put_cache.write().insert(hash, future.clone());

        future.background();

        Ok(future)
    }
}

#[derive(Debug)]
pub struct ScatterNetPutBlobInner {
    pub blob: RwLock<Option<Bytes>>,
    pub hash: Hash,
    pub hkey: RwLock<Option<Hkey>>,
    pub net: Arc<ScatterNet>,
    pub puts: RwLock<Vec<Put>>,
}

#[derive(Debug)]
pub struct Put {
    pub future: Option<Pin<Box<PeerPutBlob>>>,
    pub peer: Option<Arc<Peer>>,
    pub peer_group: Arc<PeerGroup>,
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
            puts: RwLock::new(puts),
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
}
