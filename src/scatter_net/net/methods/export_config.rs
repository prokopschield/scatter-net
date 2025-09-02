use crate::{NetConfig, ScatterNet};

impl ScatterNet {
    #[must_use]
    pub fn export_config(&self) -> NetConfig {
        self.config.clone()
    }
}
