mod implementations;
mod methods;

use std::{collections::VecDeque, sync::Arc};

use iroh::endpoint::{RecvStream, SendStream};

use super::{Packet, Peer};

#[derive(Clone, Debug)]
pub struct Interaction {
    peer: Arc<Peer>,
    recv_stream: Option<Arc<tokio::sync::Mutex<RecvStream>>>,
    send_stream: Option<Arc<tokio::sync::Mutex<SendStream>>>,
    packets: Arc<parking_lot::Mutex<VecDeque<Packet>>>,
}
