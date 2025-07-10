use crate::PeerGroup;

impl PeerGroup {
    #[must_use]
    pub const fn get_rtt_cap_ms(&self) -> u64 {
        self.config.rtt_cap_ms
    }
}
