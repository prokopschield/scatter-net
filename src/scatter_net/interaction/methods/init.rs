use std::sync::Arc;

use iroh::endpoint::{RecvStream, SendStream};

use crate::{Interaction, Peer};

impl Interaction {
    pub fn init(
        peer: Arc<Peer>,
        recv_stream: Option<RecvStream>,
        send_stream: Option<SendStream>,
    ) -> Self {
        Self {
            peer,
            recv_stream: recv_stream.map(Arc::from),
            send_stream: send_stream.map(Arc::from),
        }
    }
}
