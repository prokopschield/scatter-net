use crate::{PeerBuilder, PeerState};

impl PeerBuilder {
    #[must_use]
    pub const fn with_option_state(mut self, state: Option<PeerState>) -> Self {
        if let Some(state) = state {
            self.state = Some(state);
        }

        self
    }
}
