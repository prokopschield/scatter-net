use std::sync::Arc;

use anyhow::Result;

use crate::{PeerGroup, PeerGroupConfig, ScatterNet};

impl ScatterNet {
    pub fn init_peer_groups(self: &Arc<Self>, mut configs: Vec<PeerGroupConfig>) -> Result<()> {
        if !configs.iter().any(|peer_group| peer_group.open) {
            configs.extend_from_slice(&[
                PeerGroupConfig {
                    members: vec![],
                    name: "instant".to_string(),
                    open: true,
                    rtt_cap_ms: 4,
                },
                PeerGroupConfig {
                    members: vec![],
                    name: "local".to_string(),
                    open: true,
                    rtt_cap_ms: 16,
                },
                PeerGroupConfig {
                    members: vec![],
                    name: "near".to_string(),
                    open: true,
                    rtt_cap_ms: 64,
                },
                PeerGroupConfig {
                    members: vec![],
                    name: "midrange".to_string(),
                    open: true,
                    rtt_cap_ms: 256,
                },
                PeerGroupConfig {
                    members: vec![],
                    name: "distant".to_string(),
                    open: true,
                    rtt_cap_ms: 1024,
                },
                PeerGroupConfig {
                    members: vec![],
                    name: "far".to_string(),
                    open: true,
                    rtt_cap_ms: 4096,
                },
                PeerGroupConfig {
                    members: vec![],
                    name: "interplanetary".to_string(),
                    open: true,
                    rtt_cap_ms: u64::MAX,
                },
            ]);
        }

        let peer_groups: Result<Vec<Arc<PeerGroup>>> = configs
            .into_iter()
            .map(|config| PeerGroup::init(self.clone(), config))
            .collect();

        let mut guard = self.write();

        for peer_group in peer_groups? {
            guard.peer_groups.push(peer_group);
        }

        drop(guard);

        Ok(())
    }
}
