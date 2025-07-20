use std::task::Poll::{Pending, Ready};

use n0_future::FutureExt;

use crate::{Interaction, Packet};

impl Interaction {
    pub fn read_packet(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> Result<InteractionReadPacketResult, InteractionReadPacketError> {
        use InteractionReadPacketResult::{NoMorePackets, NoPacketYet, ReceivedPacket};

        let mut guard = self.write();

        let mut recv_stream_future = Box::pin(self.recv_stream.lock());

        let mut recv_stream = match recv_stream_future.poll(cx) {
            Pending => return Ok(NoPacketYet),
            Ready(guard) => guard,
        };

        drop(recv_stream_future);

        let mut current_offset = guard.buffer.len();

        let capacity = if current_offset >= 4 {
            u32::from_le_bytes(guard.buffer[0..4].try_into()?).try_into()?
        } else {
            guard.buffer.capacity().max(0x1000)
        };

        guard.buffer.set_len(capacity)?;

        match recv_stream.poll_read(cx, &mut guard.buffer[current_offset..]) {
            Ready(Ok(num_bytes_read)) => {
                current_offset += num_bytes_read;

                if num_bytes_read == 0 {
                    // EOF - connection closed
                    guard.buffer.truncate(current_offset);
                    drop(recv_stream);
                    drop(guard);
                    return Ok(NoMorePackets);
                }
            }
            Ready(Err(err)) => {
                guard.buffer.truncate(current_offset);
                return Err(err.into());
            }
            Pending => {
                guard.buffer.truncate(current_offset);
                return Ok(NoPacketYet);
            }
        }

        drop(recv_stream);

        guard.buffer.truncate(current_offset);

        let expected_length = if current_offset >= 4 {
            u32::from_le_bytes(guard.buffer[0..4].try_into()?).try_into()?
        } else {
            0
        };

        if current_offset < expected_length {
            drop(guard);

            // Likely returns [`Pending`]
            return self.read_packet(cx);
        }

        let packet = Packet::from_bytes(&guard.buffer[..expected_length])
            .map_err(InteractionReadPacketError::PacketFromBytes)?;

        let remainder = current_offset - expected_length;

        guard.buffer.copy_within(expected_length..current_offset, 0);
        guard.buffer.truncate(remainder);

        drop(guard);

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
