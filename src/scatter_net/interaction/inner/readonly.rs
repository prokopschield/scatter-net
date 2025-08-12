use iroh::endpoint::{RecvStream, SendStream};
use tokio::sync::Mutex;

use crate::Peer;

#[derive(Debug)]
pub struct InteractionInnerReadonly {
    pub peer: Peer,
    pub recv_stream: Mutex<RecvStream>,
    pub send_stream: Mutex<SendStream>,
}
