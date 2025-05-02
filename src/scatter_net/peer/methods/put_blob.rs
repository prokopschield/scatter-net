use std::{
    future::Future,
    sync::Arc,
    task::Poll::{Pending, Ready},
};

use bytes::Bytes;
use n0_future::StreamExt;

use crate::{spawn_and_forget, Interaction, Packet, Peer, PutRequest, PutResponse};

impl Peer {
    pub async fn put_blob(self: Arc<Self>, data: Bytes) -> Result<PeerPutBlob, PeerPutBlobError> {
        let interaction = self.begin_interaction().await?;

        let request = PutRequest { data };

        interaction.send_packet(Packet::PutRequest(request)).await?;

        Ok(PeerPutBlob { interaction })
    }
}

pub struct PeerPutBlob {
    interaction: Interaction,
}

impl Future for PeerPutBlob {
    type Output = Result<PutResponse, PeerPutBlobError>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        use PeerPutBlobError::{CannotGetNodeId, DidNotRespond, InvalidResponse};

        let this = self.get_mut();

        let packet = match this.interaction.poll_next(cx) {
            Pending => return Pending,
            Ready(None) => return Ready(Err(DidNotRespond)),
            Ready(Some(packet)) => packet,
        };

        match packet {
            Packet::PutResponse(response) => Ready(Ok(response)),
            packet => {
                let peer = this.interaction.get_peer().clone();

                let peer_node_id = match peer.node_id() {
                    Ok(node_id) => node_id,
                    Err(err) => return Ready(Err(CannotGetNodeId(err))),
                };

                eprintln!("Invalid response from {peer_node_id}: {packet:?}");

                // process the packet anyway, just in case
                spawn_and_forget(async move { Ok(packet.process(peer.clone()).await?) });

                Ready(Err(InvalidResponse))
            }
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PeerPutBlobError {
    #[error(transparent)]
    BeginInteraction(#[from] crate::PeerBeginInteractionError),
    #[error(transparent)]
    CannotGetNodeId(#[from] anyhow::Error),
    #[error("Peer did not respond to the put request.")]
    DidNotRespond,
    #[error("Peer did not provide a valid response.")]
    InvalidResponse,
    #[error(transparent)]
    SendPacket(#[from] crate::InteractionSendPacketError),
}
