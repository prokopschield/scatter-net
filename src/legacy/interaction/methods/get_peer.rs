use crate::{Interaction, Peer};

impl Interaction {
    #[must_use]
    pub fn get_peer(&self) -> &Peer {
        &self.peer
    }
}
