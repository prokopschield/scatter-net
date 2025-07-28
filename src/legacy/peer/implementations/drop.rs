use crate::{ErrorCode, PeerInnerWritable};

impl Drop for PeerInnerWritable {
    fn drop(&mut self) {
        let error_code = (ErrorCode::PeerDropped as u8).into();

        self.connection.close(error_code, b"Peer dropped.");
    }
}
