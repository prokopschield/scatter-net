mod methods;

use std::sync::Arc;

use iroh::endpoint::{RecvStream, SendStream};

use super::Peer;

#[derive(Clone, Debug)]
pub struct Interaction {
    peer: Arc<Peer>,
    recv_stream: Option<Arc<RecvStream>>,
    send_stream: Option<Arc<SendStream>>,
}
