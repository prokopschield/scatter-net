use crate::{Interaction, Peer};

impl Interaction {
    #[must_use]
    pub const fn get_peer(&self) -> &Peer {
        &self.peer
    }
}
