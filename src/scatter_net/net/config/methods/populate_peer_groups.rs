use crate::{NetConfig, PeerGroupConfig};

impl NetConfig {
    pub fn populate_peer_groups(&mut self) {
        for rtt in [50, 150, u64::MAX] {
            if !self
                .peer_groups
                .iter()
                .any(|peer_group| peer_group.open && peer_group.rtt_cap_ms >= rtt)
            {
                self.peer_groups.push(PeerGroupConfig {
                    name: format!("LT_{rtt}ms_RTT"),
                    open: true,
                    rtt_cap_ms: rtt,
                    ..Default::default()
                });
            }
        }
    }
}
