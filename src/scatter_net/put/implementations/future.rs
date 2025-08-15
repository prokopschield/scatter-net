use std::{
    future::Future,
    task::Poll::{Pending, Ready},
};

use n0_future::FutureExt;
use ps_datachunk::DataChunk;
use ps_hkey::{AsyncStore, Hkey};
use ps_promise::Promise;

use crate::{PeerGroup, Put, PutError, PutInnerWritable};

impl Future for Put {
    type Output = Result<Hkey, PutError>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let mut guard = self.write();

        if let PutInnerWritable::Done { hkey } = &*guard {
            return Ready(Ok(hkey.clone()));
        }

        if let PutInnerWritable::Initial { chunk, net } = &*guard {
            // store chunk locally
            let hkey = net.lake.put_encrypted_chunk(chunk).ok();

            // last time we need `net`
            let peer_groups = net.get_peer_groups();

            let pending: Vec<(PeerGroup, Promise<Hkey, crate::PeerGroupAsyncStoreError>)> =
                peer_groups
                    .into_iter()
                    .map(|peer_group| {
                        let chunk = chunk.clone();
                        let group = peer_group.clone();

                        (group, { Promise::new(peer_group.put(chunk.into_bytes())) })
                    })
                    .collect();

            *guard = PutInnerWritable::Processing { hkey, pending };
        }

        let PutInnerWritable::Processing { hkey, pending } = &mut *guard else {
            unreachable!("This state is logically impossible, all variants have been exhausted.");
        };

        let mut resolved = Vec::new();

        for (peer_group, mut promise) in std::mem::take(pending) {
            match promise.poll(cx) {
                Pending => pending.push((peer_group, promise)),
                Ready(result) => resolved.push((peer_group, result)),
            }
        }

        let mut error = None;

        let is_pending = !pending.is_empty();

        for (peer_group, result) in resolved {
            match result {
                Ok(value) => {
                    if hkey.is_none() {
                        hkey.replace(value);
                    }
                }
                Err(err) => {
                    if !is_pending && error.is_none() {
                        error.replace(err);
                    } else {
                        pending.push((peer_group, Promise::reject(err)));
                    }
                }
            }
        }

        if is_pending {
            return Pending;
        }

        if let Some(err) = error {
            return Ready(Err(err.into()));
        }

        // At this point, either `hkey` has a value, or everything has failed.
        let hkey = hkey.clone().ok_or(PutError::Failure)?;

        *guard = PutInnerWritable::Done { hkey: hkey.clone() };

        drop(guard);

        Ready(Ok(hkey))
    }
}
