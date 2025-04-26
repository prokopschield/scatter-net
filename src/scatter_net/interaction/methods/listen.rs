use std::sync::Arc;

use anyhow::{Error, Result};
use ps_buffer::Buffer;

use crate::{Interaction, Packet};

impl Interaction {
    pub async fn listen(self: Arc<Self>) -> Result<()> {
        let interaction = self.clone();

        let result: Result<(), Error> = async move {
            loop {
                let mut guard = interaction.recv_stream.lock().await;
                let mut length = [0u8; std::mem::size_of::<usize>()];

                guard.read_exact(&mut length[0..4]).await?;

                let length = usize::from_le_bytes(length) + 8;
                let mut buffer = Buffer::alloc_uninit(length)?;

                guard.read_exact(&mut buffer).await?;

                drop(guard);

                let packet = Packet::from_bytes(buffer)?;

                interaction.packets.lock().push_back(packet);
            }
        }
        .await;

        self.recv_stream.lock().await.stop(0u8.into())?;

        *self.closed.write() = true;

        result
    }
}
