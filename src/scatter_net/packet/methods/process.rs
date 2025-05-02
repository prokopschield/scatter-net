use std::sync::Arc;

use crate::{Packet, Peer};

impl Packet {
    pub async fn process(&self, peer: Arc<Peer>) -> Result<Option<Self>, PacketProcessError> {
        use PacketProcessError::ReceivedErrorPacket;

        match self {
            Self::Empty | Self::Pong => Ok(None),
            Self::Error => Err(ReceivedErrorPacket),
            Self::Ping => Ok(Some(Self::Pong)),
            Self::FetchRequest(_request) => todo!(),
            Self::FetchResponse(response) => response.process(peer).await,
            Self::PutRequest(_request) => todo!(),
            Self::PutResponse(_response) => todo!(),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PacketProcessError {
    #[error(transparent)]
    Put(anyhow::Error),
    #[error("The Peer sent an Error packet.")]
    ReceivedErrorPacket,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
