use crate::{Interaction, Packet};

impl Interaction {
    pub async fn send_packet(&self, packet: Packet) -> Result<(), InteractionSendPacketError> {
        let bytes = packet.to_bytes()?;

        self.send_bytes(bytes).await?;

        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum InteractionSendPacketError {
    #[error(transparent)]
    PacketToBytes(#[from] crate::PacketToBytesError),
    #[error(transparent)]
    SendBytes(#[from] crate::InteractionSendBytesError),
}
