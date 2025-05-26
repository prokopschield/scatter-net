use std::{sync::Arc, time::Duration};

use n0_future::StreamExt;
use tokio::time::sleep;

use crate::{ErrorCode, Peer, PeerGroup, Terminate};

impl Peer {
    /// Inserts this [`Peer`] into a [`PeerGroup`], if it isn't in one already.
    pub async fn select_peer_group(
        self: Arc<Self>,
    ) -> Result<Arc<PeerGroup>, PeerSelectPeerGroupError> {
        sleep(Duration::from_secs(1)).await;

        let mut interaction = self.clone().begin_interaction().await?;
        let time_start = chrono::Local::now();

        interaction.send_packet(crate::Packet::Ping).await?;

        let pong = interaction.next().await;
        let time_end = chrono::Local::now();

        if !matches!(pong, Some(Ok(crate::Packet::Pong))) {
            self.terminate(
                ErrorCode::PingPongFailed as u8,
                &"Failed to play ping pong.",
            );

            eprintln!("Failed to play ping pong with {self}, received {pong:?}.");

            return Err(PeerSelectPeerGroupError::PingPongFailed);
        }

        let rtt = (time_end - time_start).num_milliseconds().unsigned_abs();

        let mut peer_group: Option<Arc<crate::PeerGroup>> = None;

        for g in self.net().get_peer_groups() {
            if g.has_peer(&self) {
                // self already in a peer_group, we're done here
                return Ok(g);
            }

            if !g.is_open() || rtt > g.get_rtt_cap_ms() {
                // cannot join peer group
                continue;
            }

            if let Some(og) = peer_group {
                if g.get_rtt_cap_ms() < og.get_rtt_cap_ms() {
                    peer_group = Some(g);
                } else {
                    peer_group = Some(og);
                }
            } else {
                peer_group = Some(g);
            }
        }

        let node_id = self.node_id();

        peer_group.map_or_else(
            || {
                eprintln!("Could not find a suitable PeerGroup for {node_id}");

                Err(PeerSelectPeerGroupError::NoSuitablePeerGroup)
            },
            |peer_group| {
                eprintln!("Peer {node_id} inserted into {peer_group}.");

                peer_group.insert_peer(self);

                Ok(peer_group)
            },
        )
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PeerSelectPeerGroupError {
    #[error(transparent)]
    InteractionSendPacket(#[from] crate::InteractionSendPacketError),
    #[error("Could not find a suitable PeerGroup.")]
    NoSuitablePeerGroup,
    #[error(transparent)]
    PeerBeginInteraction(#[from] crate::PeerBeginInteractionError),
    #[error("Peer did not respond to Packet::Ping with Packet::Pong.")]
    PingPongFailed,
}
