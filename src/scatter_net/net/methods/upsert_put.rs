use ps_datachunk::OwnedDataChunk;
use tokio::spawn;

use crate::{Put, ScatterNet};

impl ScatterNet {
    pub(crate) fn upsert_put(&self, chunk: OwnedDataChunk) -> Put {
        let mut guard = self.write();
        let hash = chunk.hash();

        if let Some(existing) = guard.puts.get(&hash) {
            return existing.clone();
        }

        let put = Put::from_inner(
            crate::PutInnerReadonly { hash: chunk.hash() },
            crate::PutInnerWritable::Initial {
                chunk,
                net: self.clone(),
            },
        );

        guard.puts.insert(hash, put.clone());

        drop(guard);

        // propagate this Put asynchronously
        spawn(put.clone());

        put
    }
}
