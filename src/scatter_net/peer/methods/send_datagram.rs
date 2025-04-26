use anyhow::Result;
use bytes::Bytes;

use crate::Peer;

impl Peer {
    pub fn send_datagram(&self, bytes: Bytes) -> Result<()> {
        self.connection.read().send_datagram(bytes)?;

        Ok(())
    }
}
