use std::sync::Arc;

use anyhow::Result;

use crate::{spawn_and_forget, Packet, Peer};

impl Peer {
    pub async fn listen_dg(self: Arc<Self>) -> Result<()> {
        let connection = self.read().connection.clone();

        loop {
            let dg = connection.read_datagram().await?;
            let packet = Packet::from_bytes(dg)?;
            let peer_clone = self.clone();

            spawn_and_forget(async move {
                if let Some(response) = packet.process(peer_clone.clone()).await? {
                    peer_clone.send_datagram(response.to_bytes()?)?;
                }

                Ok(())
            });
        }
    }
}
