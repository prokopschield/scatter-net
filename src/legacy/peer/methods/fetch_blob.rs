use n0_future::StreamExt;
use ps_hkey::Hkey;

use crate::{FetchRequest, FetchResponse, Packet, Peer};

impl Peer {
    pub async fn fetch_blob(
        self,
        hkey: Hkey,
        level: u8,
        recursive: bool,
    ) -> Result<Option<FetchResponse>, PeerFetchBlobError> {
        let mut interaction = Self::begin_interaction(self.clone()).await?;

        let request = FetchRequest {
            hash: hkey.to_string(),
            level,
            recursive,
        };

        let packet = Packet::FetchRequest(request);

        interaction.send_packet(packet).await?;

        while let Some(response) = interaction.next().await {
            let packet = response?;

            if let Packet::FetchResponse(response) = packet {
                return Ok(Some(response));
            }

            packet.process(self.clone()).await?;
        }

        Ok(None)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PeerFetchBlobError {
    #[error(transparent)]
    BeginInteraction(#[from] crate::PeerBeginInteractionError),
    #[error(transparent)]
    PacketProcessError(#[from] crate::PacketProcessError),
    #[error("Error reading packet: {0}")]
    ReadPacket(#[from] crate::InteractionReadPacketError),
    #[error(transparent)]
    SendPacket(#[from] crate::InteractionSendPacketError),
}
