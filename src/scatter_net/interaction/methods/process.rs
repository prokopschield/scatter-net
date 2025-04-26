use n0_future::StreamExt;

use crate::{Interaction, Packet};

impl Interaction {
    pub async fn process(mut self) -> Result<(), InteractionProcessError> {
        loop {
            let Some(packet) = self.next().await else {
                return Ok(());
            };

            match packet {
                Packet::Empty | Packet::Pong => (),
                Packet::Ping => self.process_ping().await?,
                Packet::FetchRequest(_request) => todo!(),
                Packet::FetchResponse(_response) => todo!(),
                Packet::PutRequest(_request) => todo!(),
                Packet::PutResponse(_response) => todo!(),
            };
        }
    }

    async fn process_ping(&self) -> Result<(), super::InteractionSendPacketError> {
        self.send_packet(Packet::Pong).await
    }
}

#[derive(thiserror::Error, Debug)]
pub enum InteractionProcessError {
    #[error("Sending reply failed: {0}")]
    SendReply(#[from] super::InteractionSendPacketError),
}
