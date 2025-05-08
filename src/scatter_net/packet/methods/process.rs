use std::sync::Arc;

use crate::{Packet, Peer};

impl Packet {
    pub async fn process(self, peer: Arc<Peer>) -> Result<Option<Self>, PacketProcessError> {
        use PacketProcessError::ReceivedErrorPacket;

        match self {
            Self::Empty | Self::Pong => Ok(None),
            Self::Error => Err(ReceivedErrorPacket),
            Self::Ping => Ok(Some(Self::Pong)),
            Self::FetchRequest(_request) => todo!(),
            Self::FetchResponse(response) => response.process(peer).await,
            Self::PutRequest(request) => Ok(Some(
                (peer.net().put_blob(request.data)?.early_return().await).map_or_else(
                    |_| Self::PutResponse(crate::PutResponse::Failure),
                    |hkey| Self::PutResponse(crate::PutResponse::Success(hkey.to_string())),
                ),
            )),
            Self::PutResponse(response) => {
                eprintln!("Received unsolicited PutResponse({response:?}) from {peer}");
                Ok(Some(Self::Error))
            }
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PacketProcessError {
    #[error(transparent)]
    Put(#[from] crate::ScatterNetPutBlobError),
    #[error("The Peer sent an Error packet.")]
    ReceivedErrorPacket,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
