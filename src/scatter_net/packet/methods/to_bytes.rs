use anyhow::Result;
use ps_buffer::Buffer;
use ps_deflate::compress_into;

use crate::Packet;

impl Packet {
    pub fn to_bytes<B>(&self) -> Result<Buffer>
    where
        B: AsRef<[u8]>,
    {
        let serialized = bitcode::serialize(self)?;

        let ser_len = serialized.len();

        let mut buffer = Buffer::alloc_uninit(ser_len + 13)?;

        let com_len = compress_into(&serialized, &mut buffer[8..])?;

        buffer.truncate(com_len + 8);

        let com_len = u32::try_from(com_len)?;
        let ser_len = u32::try_from(ser_len)?;

        buffer[0..4].copy_from_slice(&com_len.to_le_bytes());
        buffer[4..8].copy_from_slice(&ser_len.to_le_bytes());

        Ok(buffer)
    }
}
