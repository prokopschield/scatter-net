use crate::Peer;

impl Peer {
    pub fn is_available(&self) -> bool {
        !self.state.read().terminated
    }
}
