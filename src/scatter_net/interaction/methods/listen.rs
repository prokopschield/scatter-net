use std::sync::Arc;

use anyhow::Result;
use ps_buffer::Buffer;

use crate::{Interaction, Packet};

impl Interaction {
    pub async fn listen(self: Arc<Self>) -> Result<()> {
        loop {
            if let Some(mutex) = &self.recv_stream {
                let mut guard = mutex.lock().await;
                let mut length = [0u8; std::mem::size_of::<usize>()];

                guard.read_exact(&mut length[0..4]).await?;

                let length = usize::from_le_bytes(length) + 8;
                let mut buffer = Buffer::alloc_uninit(length)?;

                guard.read_exact(&mut buffer).await?;

                drop(guard);

                let packet = Packet::from_bytes(buffer)?;

                self.packets.lock().push_back(packet);
            }
        }
    }
}
