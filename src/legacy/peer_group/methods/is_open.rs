use crate::PeerGroup;

impl PeerGroup {
    #[must_use]
    pub const fn is_open(&self) -> bool {
        self.config.open
    }
}
