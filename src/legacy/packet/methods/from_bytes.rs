use anyhow::Result;
use ps_deflate::decompress;

use crate::Packet;

impl Packet {
    pub fn from_bytes<B>(bytes: B) -> Result<Self>
    where
        B: AsRef<[u8]>,
    {
        let bytes = bytes.as_ref();

        let size = bytes.get(4..8).unwrap_or(&[]).try_into()?;
        let size = usize::try_from(u32::from_le_bytes(size))?;

        let bytes = decompress(&bytes[8..], size)?;

        let packet = bitcode::deserialize(&bytes)?;

        Ok(packet)
    }
}
