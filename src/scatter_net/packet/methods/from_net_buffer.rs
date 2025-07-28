use crate::Packet;

impl Packet {
    /// Reads the first packet from a network buffer.
    pub fn from_net_buffer(
        bytes: &[u8],
    ) -> Result<Option<(usize, Self)>, PacketFromNetBufferError> {
        if bytes.len() < 8 {
            return Ok(None);
        }

        let length = usize::try_from(u32::from_be_bytes(bytes[0..4].try_into()?))?;

        if bytes.len() < length {
            return Ok(None);
        }

        let packet = Self::from_bytes(&bytes[..length])?;

        Ok(Some((length, packet)))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PacketFromNetBufferError {
    #[error(transparent)]
    FromBytes(#[from] anyhow::Error),
    #[error(transparent)]
    Int(#[from] std::num::TryFromIntError),
    #[error(transparent)]
    Slice(#[from] std::array::TryFromSliceError),
}

#[cfg(test)]
mod tests {
    use anyhow::{bail, Result};
    use ps_buffer::Buffer;

    use crate::Packet;

    #[test]
    fn roundtrip() -> Result<()> {
        let packet = Packet::Pong;
        let serialized = packet.to_bytes()?;

        let mut buffer = Buffer::alloc_uninit(200_000)?;

        buffer[0..serialized.len()].copy_from_slice(&serialized);

        if let Some((size, dpacket)) = Packet::from_net_buffer(&buffer)? {
            assert_eq!(packet, dpacket, "Packets should match");
            assert_eq!(size, serialized.len(), "Packet lengths should match");
        } else {
            bail!("Packet deserialization failed");
        }

        Ok(())
    }
}
