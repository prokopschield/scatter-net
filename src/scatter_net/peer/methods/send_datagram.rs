use bytes::Bytes;
use iroh::endpoint::SendDatagramError;

use crate::Peer;

impl Peer {
    pub fn send_datagram(&self, bytes: Bytes) -> Result<(), SendDatagramError> {
        self.connection.read().send_datagram(bytes)
    }
}
