use std::sync::Arc;

use crate::{Interaction, Peer};

impl Interaction {
    #[must_use]
    pub const fn get_peer(&self) -> &Arc<Peer> {
        &self.peer
    }
}
