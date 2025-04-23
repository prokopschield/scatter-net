use iroh::endpoint::VarInt;

use crate::Peer;

impl Peer {
    pub fn terminate<E, R>(peer: &Self, error_code: E, reason: &R)
    where
        E: Into<VarInt> + Send,
        R: AsRef<[u8]> + Send,
    {
        peer.connection.close(error_code.into(), reason.as_ref());

        let mut state = peer.state.write();

        state.terminated = true;
    }
}
