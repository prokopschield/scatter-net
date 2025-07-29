use crate::{PeerBuilder, PeerState};

impl PeerBuilder {
    #[must_use]
    pub const fn with_state(mut self, state: PeerState) -> Self {
        self.state = Some(state);

        self
    }
}
