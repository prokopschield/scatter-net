use crate::{ErrorCode, Peer};

impl Drop for Peer {
    fn drop(&mut self) {
        let error_code = (ErrorCode::PeerDropped as u8).into();

        self.connection.read().close(error_code, b"Peer dropped.");
    }
}
