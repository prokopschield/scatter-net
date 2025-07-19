mod implementations;
mod methods;

use std::{collections::VecDeque, sync::Arc};

use iroh::endpoint::{RecvStream, SendStream};
pub use methods::*;
use parking_lot::RwLock;
use ps_buffer::Buffer;

use crate::{Packet, Peer};

#[derive(Clone, Debug)]
pub struct Interaction {
    buffer: Arc<RwLock<Buffer>>,
    peer: Peer,
    recv_stream: Arc<tokio::sync::Mutex<RecvStream>>,
    send_stream: Option<Arc<tokio::sync::Mutex<SendStream>>>,
    packets: Arc<parking_lot::Mutex<VecDeque<Packet>>>,
    closed: Arc<RwLock<bool>>,
}
