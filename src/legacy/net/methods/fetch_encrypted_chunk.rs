use std::{
    collections::VecDeque,
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{
        Poll::{Pending, Ready},
        Waker,
    },
};

use anyhow::Result;
use n0_future::FutureExt;
use parking_lot::RwLock;
use ps_buffer::BufferError;
use ps_datachunk::{DataChunk, SerializedDataChunk};
use ps_datalake::error::PsDataLakeError;
use ps_hkey::{Hash, Hkey};
use tokio::task::JoinHandle;

use crate::{FetchResponse, Peer, PeerGroup, ScatterNet};

impl ScatterNet {
    #[must_use]
    pub fn fetch_encrypted_chunk(&self, hash: Arc<Hash>) -> ScatterNetFetchEncryptedChunk {
        ScatterNetFetchEncryptedChunk::init(self, hash)
    }
}

type SerializedDataChunkFuture = dyn Future<Output = Option<SerializedDataChunk>> + Send + Sync;
type BoxedSerializedDataChunkFuture = Pin<Box<SerializedDataChunkFuture>>;

pub struct ScatterNetFetchEncryptedChunk<'lt> {
    futures: RwLock<Vec<BoxedSerializedDataChunkFuture>>,
    net: &'lt ScatterNet,
    hash: Arc<Hash>,
    peer_groups: VecDeque<Arc<PeerGroup>>,
    value: Option<SerializedDataChunk>,
    timeout: Option<JoinHandle<()>>,
    num_attempts_all_peers: u8,
}

impl<'lt> ScatterNetFetchEncryptedChunk<'lt> {
    pub fn init(net: &'lt ScatterNet, hash: Arc<Hash>) -> Self {
        let locally_found = match net.lake.get_encrypted_chunk(&hash) {
            Ok(chunk) => chunk.serialize().ok(),
            Err(PsDataLakeError::NotFound) => None,
            Err(err) => {
                eprintln!("Fetching chunk {hash} from DataLake failed: {err:?}");
                None
            }
        };

        let peer_groups = match &locally_found {
            None => net.read().peer_groups.clone().into(),
            Some(_) => VecDeque::new(),
        };

        Self {
            futures: RwLock::default(),
            hash,
            net,
            peer_groups,
            value: locally_found,
            timeout: None,
            num_attempts_all_peers: 0,
        }
    }

    pub fn schedule(&mut self, waker: Waker) {
        let new_task = tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            waker.wake();
        });

        if let Some(old_task) = self.timeout.replace(new_task) {
            old_task.abort();
        }
    }
}

impl Future for ScatterNetFetchEncryptedChunk<'_> {
    type Output = Result<SerializedDataChunk, ScatterNetFetchEncryptedChunkError>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let this = self.get_mut();
        let hash = this.hash.clone();

        this.futures
            .write()
            .retain_mut(|future| match future.poll(cx) {
                std::task::Poll::Pending => true,
                std::task::Poll::Ready(None) => false,
                std::task::Poll::Ready(Some(chunk)) => {
                    this.value = Some(chunk);
                    false
                }
            });

        if let Some(chunk) = this.value.take() {
            return Ready(Ok(chunk));
        }

        let mut request_from_peer = |peer: Peer, hash: Arc<Hash>| {
            let mut future = Box::pin(async move {
                let fetched = Peer::fetch_blob(peer, Hkey::Direct(hash)).await;

                match fetched {
                    Ok(Some(FetchResponse::Success(buffer))) => {
                        SerializedDataChunk::from_data(buffer).ok()
                    }
                    _ => None,
                }
            });

            match future.poll(cx) {
                Pending => {
                    this.futures.write().push(future);
                }
                Ready(Some(chunk)) => {
                    this.value.replace(chunk);
                }
                Ready(None) => (),
            }
        };

        if let Some(peer_group) = this.peer_groups.pop_front() {
            if let Some(peer) = peer_group.get_peer_by_hash(&this.hash) {
                request_from_peer(peer, hash.clone());
            }
        }

        if this.futures.read().is_empty() && this.peer_groups.is_empty() {
            if this.num_attempts_all_peers >= 3 {
                return Ready(Err(ScatterNetFetchEncryptedChunkError::NotFound));
            }

            let peers: Vec<Peer> = this.net.read().peers.values().cloned().collect();

            for peer in peers {
                request_from_peer(peer, hash.clone());
            }

            this.num_attempts_all_peers += 1;
        }

        this.schedule(cx.waker().clone());

        Pending
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ScatterNetFetchEncryptedChunkError {
    #[error(transparent)]
    BufferError(#[from] BufferError),
    #[error("Unable to fetch the data in question.")]
    NotFound,
}
