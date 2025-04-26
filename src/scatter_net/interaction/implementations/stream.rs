use anyhow::Result;
use n0_future::Stream;

use crate::{Interaction, Packet};

impl Stream for Interaction {
    type Item = Result<Packet>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let packet = self.packets.lock().pop_front();

        packet.map_or_else(
            || {
                if *self.closed.read() {
                    std::task::Poll::Ready(None)
                } else {
                    std::task::Poll::Pending
                }
            },
            |packet| std::task::Poll::Ready(Some(Ok(packet))),
        )
    }
}
