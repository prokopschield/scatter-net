use std::collections::VecDeque;

use ps_datachunk::OwnedDataChunk;
use ps_promise::Promise;

use crate::{PeerGroup, PeerGroupAsyncStoreError, ScatterNet};

#[derive(Debug)]
pub enum FetchInnerWritable {
    Initial {
        net: ScatterNet,
    },
    Fetching {
        net: ScatterNet,
        peer_groups: VecDeque<PeerGroup>,
        promises: Vec<Promise<OwnedDataChunk, PeerGroupAsyncStoreError>>,
    },
    Done {
        chunk: OwnedDataChunk,
    },
}
