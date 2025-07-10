use n0_future::StreamExt;

use crate::Interaction;

impl Interaction {
    pub async fn process(mut self) -> Result<(), InteractionProcessError> {
        loop {
            match self.next().await {
                Some(Ok(packet)) => {
                    if let Some(response) = packet.process(self.peer.clone()).await? {
                        self.send_packet(response).await?;
                    }
                }
                Some(Err(err)) => return Err(err)?,
                None => return Ok(()),
            }
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum InteractionProcessError {
    #[error("Processing packet failed: {0}")]
    PacketProcessError(#[from] crate::PacketProcessError),
    #[error("Reading packet failed: {0}")]
    ReadPacket(#[from] crate::InteractionReadPacketError),
    #[error("Sending reply failed: {0}")]
    SendReply(#[from] super::InteractionSendPacketError),
}
