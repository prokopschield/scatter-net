use std::collections::VecDeque;

use iroh::endpoint::{RecvStream, SendStream};
use ps_buffer::Buffer;
use tokio::sync::Mutex;

use crate::{Interaction, InteractionInnerReadonly, InteractionInnerWritable, Peer};

impl Interaction {
    #[must_use]
    pub fn init(peer: Peer, recv_stream: RecvStream, send_stream: SendStream) -> Self {
        Self::from_inner(
            InteractionInnerReadonly {
                peer,
                recv_stream: Mutex::new(recv_stream),
                send_stream: Mutex::new(send_stream),
            },
            InteractionInnerWritable {
                buffer: Buffer::default(),
                closed: false,
                packets: VecDeque::new(),
            },
        )
    }
}
