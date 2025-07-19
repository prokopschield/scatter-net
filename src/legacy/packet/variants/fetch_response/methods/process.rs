use ps_str::Utf8Encoder;

use crate::{FetchResponse, Packet, PacketProcessError, Peer, ScatterNet};

impl FetchResponse {
    pub async fn process(self, peer: Peer) -> Result<Option<Packet>, PacketProcessError> {
        match self {
            Self::Error => {
                eprintln!(
                    "Received spurious FetchResponse::Error from {}",
                    peer.node_id()
                );
            }
            Self::NotFound => {
                eprintln!(
                    "Received spurious FetchResponse::NotFound from {}",
                    peer.node_id()
                );
            }
            Self::Success(blob) => {
                let hkey = ScatterNet::put_blob(peer.net(), blob)?.await?;

                let hkey = hkey.to_string();

                return Ok(Some(Packet::PutResponse(crate::PutResponse::Success(hkey))));
            }
            Self::Suggest(node_id) => {
                eprintln!(
                    "Received spurious FetchResponse::Suggest({}) from {}",
                    node_id.to_utf8_string(),
                    peer.node_id(),
                );
            }
        }

        Ok(None)
    }
}
