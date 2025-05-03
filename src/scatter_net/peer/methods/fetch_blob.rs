use std::sync::Arc;

use n0_future::StreamExt;
use ps_hkey::Hkey;

use crate::{FetchRequest, FetchResponse, Packet, Peer};

impl Peer {
    pub async fn fetch_blob(
        self: Arc<Self>,
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
            if let Packet::FetchResponse(response) = response {
                return Ok(Some(response));
            }

            response.process(self.clone()).await?;
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
    #[error(transparent)]
    SendPacket(#[from] crate::InteractionSendPacketError),
}
