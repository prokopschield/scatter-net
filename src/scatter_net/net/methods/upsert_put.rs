use ps_datachunk::OwnedDataChunk;

use crate::{Put, ScatterNet};

impl ScatterNet {
    fn upsert_put(&self, chunk: OwnedDataChunk) -> Put {
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

        put
    }
}
