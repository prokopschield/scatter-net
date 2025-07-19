use iroh::endpoint::VarInt;

pub trait Terminate<E, R>
where
    E: Into<VarInt> + Send,
    R: AsRef<[u8]> + Send,
{
    fn terminate(&self, error_code: E, reason: &R);
}
