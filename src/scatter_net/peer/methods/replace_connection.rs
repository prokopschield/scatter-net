use anyhow::Result;
use iroh::endpoint::Connection;

use crate::Peer;

impl Peer {
    pub fn replace_connection(&self, connection: Connection) -> Result<()> {
        *self.connection.write() = connection;

        eprintln!("Replaced connection for {}", self.node_id()?);

        Ok(())
    }
}
