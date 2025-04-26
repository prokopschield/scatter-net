use std::sync::Arc;

use anyhow::Result;

use crate::{spawn_and_forget, Packet, Peer};

impl Peer {
    pub async fn listen_dg(peer: Arc<Self>) -> Result<()> {
        let connection = peer.connection.read().clone();

        loop {
            let dg = connection.read_datagram().await?;
            let packet = Packet::from_bytes(dg)?;
            let peer_clone = peer.clone();

            spawn_and_forget(async move {
                if let Some(response) = packet.process(peer_clone.clone()).await? {
                    peer_clone.send_datagram(response.to_bytes()?)?;
                }

                Ok(())
            });
        }
    }
}
