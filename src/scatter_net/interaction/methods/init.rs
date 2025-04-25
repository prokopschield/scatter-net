use std::sync::Arc;

use iroh::endpoint::{RecvStream, SendStream};

use crate::{Interaction, Peer};

impl Interaction {
    pub fn init(
        peer: Arc<Peer>,
        recv_stream: Option<RecvStream>,
        send_stream: Option<SendStream>,
    ) -> Interaction {
        Interaction {
            peer,
            recv_stream: recv_stream.map(|stream| Arc::from(stream)),
            send_stream: send_stream.map(|stream| Arc::from(stream)),
        }
    }
}
