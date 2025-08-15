use ps_datachunk::OwnedDataChunk;
use ps_hkey::Hkey;
use ps_promise::Promise;

use crate::{PeerGroup, PeerGroupAsyncStoreError, ScatterNet};

#[derive(Debug)]
pub enum PutInnerWritable {
    Initial {
        chunk: OwnedDataChunk,
        net: ScatterNet,
    },
    Processing {
        hkey: Option<Hkey>,
        pending: Vec<(PeerGroup, Promise<Hkey, PeerGroupAsyncStoreError>)>,
    },
    Done {
        hkey: Hkey,
    },
}
