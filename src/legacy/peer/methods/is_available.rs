use crate::Peer;

impl Peer {
    #[must_use]
    pub fn is_available(&self) -> bool {
        !self.read().state.terminated
    }
}
