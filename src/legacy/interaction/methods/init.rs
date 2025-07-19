use std::sync::Arc;

use iroh::endpoint::{RecvStream, SendStream};
use tokio::sync::Mutex;

use crate::{Interaction, Peer};

impl Interaction {
    pub fn init(peer: Peer, recv_stream: RecvStream, send_stream: Option<SendStream>) -> Self {
        Self {
            buffer: Arc::default(),
            peer,
            recv_stream: Arc::new(Mutex::new(recv_stream)),
            send_stream: send_stream.map(Mutex::new).map(Arc::new),
            packets: Arc::default(),
            closed: Arc::default(),
        }
    }
}
