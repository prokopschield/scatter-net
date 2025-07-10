use std::sync::Arc;

use iroh::endpoint::VarInt;

pub trait Terminate<E, R>
where
    E: Into<VarInt> + Send,
    R: AsRef<[u8]> + Send,
{
    fn terminate(self: &Arc<Self>, error_code: E, reason: &R);
}
