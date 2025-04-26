mod implementations;
mod methods;

pub use methods::*;

use std::{collections::VecDeque, sync::Arc};

use iroh::endpoint::{RecvStream, SendStream};
use parking_lot::RwLock;

use super::{Packet, Peer};

#[derive(Clone, Debug)]
pub struct Interaction {
    peer: Arc<Peer>,
    recv_stream: Arc<tokio::sync::Mutex<RecvStream>>,
    send_stream: Option<Arc<tokio::sync::Mutex<SendStream>>>,
    packets: Arc<parking_lot::Mutex<VecDeque<Packet>>>,
    closed: Arc<RwLock<bool>>,
}
