use std::task::Poll::{Pending, Ready};

use n0_future::Stream;

use crate::{InteractionReadPacketError, InteractionReadPacketResult};

use crate::{Interaction, Packet};

impl Stream for Interaction {
    type Item = Result<Packet, InteractionReadPacketError>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        use InteractionReadPacketResult::{NoMorePackets, NoPacketYet, ReceivedPacket};

        let this = self.get_mut();

        match this.read_packet(cx) {
            Ok(NoPacketYet) => Pending,
            Ok(NoMorePackets) => Ready(None),
            Ok(ReceivedPacket(packet)) => Ready(Some(Ok(packet))),
            Err(err) => Ready(Some(Err(err))),
        }
    }
}
