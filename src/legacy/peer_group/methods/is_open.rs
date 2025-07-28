use crate::PeerGroup;

impl PeerGroup {
    #[must_use]
    pub fn is_open(&self) -> bool {
        self.read().config.open
    }
}
