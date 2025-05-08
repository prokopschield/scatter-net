use std::task::Poll::{Pending, Ready};

use n0_future::FutureExt;

use crate::{Interaction, Packet};

impl Interaction {
    pub fn read_packet(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> Result<InteractionReadPacketResult, InteractionReadPacketError> {
        use InteractionReadPacketResult::{NoPacketYet, ReceivedPacket};

        let mut buffer = self.buffer.write();

        let mut recv_stream_future = Box::pin(self.recv_stream.lock());

        let mut recv_stream = match recv_stream_future.poll(cx) {
            Pending => return Ok(NoPacketYet),
            Ready(guard) => guard,
        };

        drop(recv_stream_future);

        let mut current_offset = buffer.len();

        let capacity = if current_offset >= 4 {
            u32::from_le_bytes(buffer[0..4].try_into()?).try_into()?
        } else {
            buffer.capacity().max(0x1000)
        };

        buffer.set_len(capacity)?;

        match recv_stream.poll_read(cx, &mut buffer[current_offset..]) {
            Ready(Ok(num_bytes_read)) => {
                current_offset += num_bytes_read;

                if num_bytes_read == 0 {}
            }
            Ready(Err(err)) => {
                buffer.truncate(current_offset);
                return Err(err.into());
            }
            Pending => {
                buffer.truncate(current_offset);
                return Ok(NoPacketYet);
            }
        }

        drop(recv_stream);

        buffer.truncate(current_offset);

        let expected_length = if current_offset >= 4 {
            u32::from_le_bytes(buffer[0..4].try_into()?).try_into()?
        } else {
            0
        };

        if current_offset < expected_length {
            drop(buffer);

            // Likely returns [`Pending`]
            return self.read_packet(cx);
        };

        let packet = Packet::from_bytes(&buffer[..expected_length])
            .map_err(InteractionReadPacketError::PacketFromBytes)?;

        let remainder = current_offset - expected_length;

        buffer.copy_within(expected_length..current_offset, 0);
        buffer.truncate(remainder);

        drop(buffer);

        Ok(ReceivedPacket(packet))
    }
}

pub enum InteractionReadPacketResult {
    NoMorePackets,
    NoPacketYet,
    ReceivedPacket(Packet),
}

#[derive(thiserror::Error, Debug)]
pub enum InteractionReadPacketError {
    #[error(transparent)]
    Buffer(#[from] ps_buffer::BufferError),
    #[error(transparent)]
    PacketFromBytes(anyhow::Error),
    #[error(transparent)]
    Read(#[from] iroh::endpoint::ReadError),
    #[error(transparent)]
    TryFromInt(#[from] std::num::TryFromIntError),
    #[error(transparent)]
    TryFromSlice(#[from] std::array::TryFromSliceError),
}
