use crate::{NetState, ScatterNet};

impl ScatterNet {
    #[must_use]
    pub fn export_state(&self) -> NetState {
        self.read().state.clone()
    }
}
