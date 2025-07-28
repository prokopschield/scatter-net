use std::collections::HashMap;

use anyhow::Result;

use crate::{PeerGroup, PeerGroupConfig, ScatterNet};

impl PeerGroup {
    pub fn init(net: ScatterNet, config: PeerGroupConfig) -> Result<Self> {
        let peer_group = Self::from_inner(
            crate::PeerGroupInnerReadonly { net },
            crate::PeerGroupInnerWritable {
                config,
                peers: HashMap::default(),
            },
        );

        Ok(peer_group)
    }
}
