use anyhow::Result;

use crate::NetConfig;

impl NetConfig {
    pub fn populate(&mut self) -> Result<()> {
        self.populate_lake_config()?;
        self.populate_peer_groups();
        self.populate_secret_key();

        Ok(())
    }
}
