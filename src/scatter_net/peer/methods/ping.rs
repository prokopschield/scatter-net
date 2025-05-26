use std::{sync::Arc, time::Duration};

use chrono::TimeDelta;
use n0_future::StreamExt;
use tokio::time::timeout;

use crate::Peer;

impl Peer {
    /// Sends a ping packet to the peer and measures the round-trip time.
    ///
    /// # Arguments
    /// * `timeout_duration` - Maximum time to wait for a pong response
    ///
    /// # Returns
    /// * `Ok(TimeDelta)` - Round-trip time
    /// * `Err(PeerPingError)` - Various failure modes
    pub async fn ping(
        self: Arc<Self>,
        timeout_duration: Duration,
    ) -> Result<TimeDelta, PeerPingError> {
        let mut interaction = self.begin_interaction().await?;

        let time_start = chrono::Local::now();

        interaction.send_packet(crate::Packet::Ping).await?;

        let response = match timeout(timeout_duration, interaction.next()).await {
            Ok(Some(result)) => result?,
            Ok(None) => Err(PeerPingError::ConnectionClosed)?,
            Err(_) => Err(PeerPingError::Timeout {
                timeout_ms: timeout_duration.as_millis().try_into()?,
            })?,
        };

        let time_end = chrono::Local::now();

        match response {
            crate::Packet::Pong => Ok(time_end - time_start),
            packet => Err(PeerPingError::UnexpectedResponse { received: packet }),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PeerPingError {
    #[error(transparent)]
    BeginInteraction(#[from] crate::PeerBeginInteractionError),
    #[error("Connection closed before receiving response")]
    ConnectionClosed,
    #[error("Reading packet failed: {0}")]
    ReadPacket(#[from] crate::InteractionReadPacketError),
    #[error("Sending packet failed: {0}")]
    SendPacket(#[from] crate::InteractionSendPacketError),
    #[error("Ping timed out after {timeout_ms}ms")]
    Timeout { timeout_ms: u64 },
    #[error("Expected Pong packet, received {received:?}")]
    UnexpectedResponse { received: crate::Packet },
    #[error(transparent)]
    IntConversion(#[from] std::num::TryFromIntError),
}
