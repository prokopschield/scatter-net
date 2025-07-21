use std::{
    array::TryFromSliceError,
    num::TryFromIntError,
    task::{
        Context,
        Poll::{Pending, Ready},
    },
};

use iroh::endpoint::ReadError;
use n0_future::FutureExt;
use ps_buffer::BufferError;

use crate::{Interaction, Packet, PacketFromNetBufferError};

impl Interaction {
    pub fn read_packet_new(&self, cx: &mut Context<'_>) -> InteractionReadPacketResult {
        let mut guard = self.write();

        let mut recv_stream = match Box::pin(self.recv_stream.lock()).poll(cx) {
            Pending => return Err(InteractionReadPacketError::RecvStreamLocked),
            Ready(recv_stream) => recv_stream,
        };

        loop {
            let mut current_offset = guard.buffer.len();

            let capacity = {
                if current_offset >= 4 {
                    let capacity_bytes = &guard.buffer[0..4];
                    let capacity = u32::from_be_bytes(capacity_bytes.try_into()?);

                    usize::try_from(capacity)?
                } else {
                    guard.buffer.capacity().max(0x1000)
                }
            };

            guard.buffer.set_len(capacity)?;

            let result = recv_stream.poll_read(cx, &mut guard.buffer[current_offset..]);

            match result {
                Pending => {
                    guard.buffer.truncate(current_offset);

                    return match guard.packets.pop_front() {
                        Some(packet) => Ok(InteractionReadPacket::Packet(packet)),
                        None => Ok(InteractionReadPacket::Waiting),
                    };
                }

                Ready(Err(err)) => {
                    guard.buffer.truncate(current_offset);

                    return match guard.packets.pop_front() {
                        Some(packet) => Ok(InteractionReadPacket::Packet(packet)),
                        None => Err(err)?,
                    };
                }

                Ready(Ok(bytes_read)) => {
                    current_offset += bytes_read;
                    guard.buffer.truncate(current_offset);

                    while let Some((size, packet)) = Packet::from_net_buffer(&guard.buffer)? {
                        guard.packets.push_back(packet);
                        guard.buffer.copy_within(size..current_offset, 0);
                        current_offset -= size;
                        guard.buffer.truncate(current_offset);
                    }

                    if bytes_read == 0 {
                        return match guard.packets.pop_front() {
                            Some(packet) => Ok(InteractionReadPacket::Packet(packet)),
                            None => Ok(InteractionReadPacket::EOF),
                        };
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum InteractionReadPacket {
    EOF,
    Packet(Packet),
    Waiting,
}

#[derive(thiserror::Error, Debug)]
pub enum InteractionReadPacketError {
    #[error(transparent)]
    Buffer(#[from] BufferError),
    #[error(transparent)]
    Parse(#[from] PacketFromNetBufferError),
    #[error(transparent)]
    Read(#[from] ReadError),
    #[error("The mutex on this interaction's recv_stream was locked.")]
    RecvStreamLocked,
    #[error(transparent)]
    TryFromInt(#[from] TryFromIntError),
    #[error(transparent)]
    TryFromSlice(#[from] TryFromSliceError),
}

pub type InteractionReadPacketResult = Result<InteractionReadPacket, InteractionReadPacketError>;
