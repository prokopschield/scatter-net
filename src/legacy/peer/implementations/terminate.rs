use std::sync::Arc;

use iroh::endpoint::VarInt;

use crate::{Peer, Terminate};

impl<E, R> Terminate<E, R> for Peer
where
    E: Into<VarInt> + Send,
    R: AsRef<[u8]> + Send,
{
    fn terminate(self: &Arc<Self>, error_code: E, reason: &R) {
        self.connection
            .read()
            .close(error_code.into(), reason.as_ref());

        let mut state = self.state.write();

        state.terminated = true;
    }
}
