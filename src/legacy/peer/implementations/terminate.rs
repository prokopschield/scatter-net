use iroh::endpoint::VarInt;

use crate::{Peer, Terminate};

impl<E, R> Terminate<E, R> for Peer
where
    E: Into<VarInt> + Send,
    R: AsRef<[u8]> + Send,
{
    fn terminate(&self, error_code: E, reason: &R) {
        self.read()
            .connection
            .close(error_code.into(), reason.as_ref());

        let mut guard = self.write();

        guard.state.terminated = true;
    }
}
