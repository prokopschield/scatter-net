use crate::PeerGroup;

impl PeerGroup {
    #[must_use]
    pub fn get_rtt_cap_ms(&self) -> u64 {
        self.read().config.rtt_cap_ms
    }
}
