use bytes::Bytes;
use ps_hkey::{AsyncStore, Hkey};
use ps_promise::Promise;

use crate::{ScatterNet, ScatterNetAsyncStoreError};

impl ScatterNet {
    pub fn put_blob(&self, data: Bytes) -> Promise<Hkey, ScatterNetAsyncStoreError> {
        self.put(data)
    }
}
