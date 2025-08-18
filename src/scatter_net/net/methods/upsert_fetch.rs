use std::sync::Arc;

use ps_hash::Hash;

use crate::{Fetch, ScatterNet};

impl ScatterNet {
    pub(crate) fn upsert_fetch(&self, hash: Arc<Hash>) -> Fetch {
        let mut guard = self.write();

        if let Some(existing) = guard.fetches.get(&hash) {
            return existing.clone();
        }

        let fetch = Fetch::from_inner(
            crate::FetchInnerReadonly { hash: hash.clone() },
            crate::FetchInnerWritable::Initial { net: self.clone() },
        );

        guard.fetches.insert(hash, fetch.clone());

        drop(guard);

        fetch
    }
}
