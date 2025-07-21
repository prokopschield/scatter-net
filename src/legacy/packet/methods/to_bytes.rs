use bytes::Bytes;
use ps_buffer::Buffer;
use ps_deflate::compress_into;

use crate::Packet;

impl Packet {
    pub fn to_bytes(&self) -> Result<Bytes, PacketToBytesError> {
        let serialized = bitcode::serialize(self)?;

        let ser_len = serialized.len();

        let mut buffer = Buffer::alloc_uninit(ser_len + 13)?;

        let com_len = 8 + compress_into(&serialized, &mut buffer[8..])?;

        buffer.truncate(com_len);

        let com_len = u32::try_from(com_len)?;
        let ser_len = u32::try_from(ser_len)?;

        buffer[0..4].copy_from_slice(&com_len.to_le_bytes());
        buffer[4..8].copy_from_slice(&ser_len.to_le_bytes());

        let bytes = Bytes::from_owner(buffer.share());

        Ok(bytes)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PacketToBytesError {
    #[error("Buffer allocation failed: {0}")]
    Buffer(#[from] ps_buffer::BufferError),
    #[error("Compression failed: {0}")]
    Compression(#[from] ps_deflate::PsDeflateError),
    #[error("Integer conversion failed: {0}")]
    IntConversion(#[from] std::num::TryFromIntError),
    #[error("bitcode serialization failed: {0}")]
    Serialization(#[from] bitcode::Error),
}
