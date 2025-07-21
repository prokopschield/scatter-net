use std::task::Poll::{Pending, Ready};

use n0_future::Stream;

use crate::{Interaction, InteractionReadPacket, InteractionReadPacketError, Packet};

impl Stream for Interaction {
    type Item = Result<Packet, InteractionReadPacketError>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let this = self.get_mut();

        match this.read_packet(cx) {
            Ok(InteractionReadPacket::EOF) => Ready(None),
            Ok(InteractionReadPacket::Packet(packet)) => Ready(Some(Ok(packet))),
            Ok(InteractionReadPacket::Waiting) => Pending,
            Err(err) => Ready(Some(Err(err))),
        }
    }
}
