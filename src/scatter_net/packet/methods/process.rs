use std::sync::Arc;

use crate::{Packet, Peer};

impl Packet {
    pub async fn process(&self, peer: Arc<Peer>) -> Result<Option<Self>, PacketProcessError> {
        match self {
            Self::Empty | Self::Pong => Ok(None),
            Self::Ping => Ok(Some(Self::Pong)),
            Self::FetchRequest(_request) => todo!(),
            Self::FetchResponse(_response) => todo!(),
            Self::PutRequest(_request) => todo!(),
            Self::PutResponse(_response) => todo!(),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PacketProcessError {}
