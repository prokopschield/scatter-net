use std::sync::Arc;

use anyhow::Result;

use crate::{PeerGroup, PeerGroupConfig, ScatterNet};

impl PeerGroup {
    pub fn init(net: Arc<ScatterNet>, config: PeerGroupConfig) -> Result<Arc<Self>> {
        let peer_group = Self {
            config,
            net,
            peers: Arc::default(),
        };

        Ok(Arc::new(peer_group))
    }
}
