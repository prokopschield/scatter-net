use std::{
    future::Future,
    task::Poll::{Pending, Ready},
};

use n0_future::FutureExt;
use ps_datachunk::{DataChunk, OwnedDataChunk};
use ps_hkey::AsyncStore;

use crate::{Fetch, FetchError, FetchInnerWritable};

impl Future for Fetch {
    type Output = Result<OwnedDataChunk, FetchError>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let mut guard = self.write();

        if let FetchInnerWritable::Done { chunk } = &*guard {
            return Ready(Ok(chunk.clone()));
        }

        if let FetchInnerWritable::Initial { net } = &*guard {
            // try getting chunk locally
            if let Ok(chunk) = net.lake.get_encrypted_chunk(&self.hash) {
                let chunk = chunk.into_owned();

                *guard = FetchInnerWritable::Done {
                    chunk: chunk.clone(),
                };

                return Ready(Ok(chunk));
            }

            let peer_groups = net.get_peer_groups().into();

            *guard = FetchInnerWritable::Fetching {
                net: net.clone(), // TODO optimize out this clone eventually
                peer_groups,
                promises: Vec::new(),
            };
        }

        let FetchInnerWritable::Fetching {
            net,
            peer_groups,
            promises,
        } = &mut *guard
        else {
            unreachable!("This state is logically impossible, all variants have been exhausted.");
        };

        for mut promise in std::mem::take(promises) {
            match promise.poll(cx) {
                Pending => promises.push(promise),
                Ready(Ok(chunk)) => {
                    net.upsert_put(chunk.clone());

                    *guard = FetchInnerWritable::Done {
                        chunk: chunk.clone(),
                    };

                    drop(guard); // allow Future to be polled again

                    return Ready(Ok(chunk));
                }
                Ready(Err(_)) => (),
            }
        }

        if let Some(peer_group) = peer_groups.pop_front() {
            let mut promise = peer_group.get(&self.hash);

            // poll the Future synchronously (send request)
            promise.ready(cx);

            promises.push(promise);

            return Pending;
        }

        if promises.is_empty() {
            Ready(Err(FetchError::OptionsExhausted(self.hash.clone())))
        } else {
            Pending
        }
    }
}
