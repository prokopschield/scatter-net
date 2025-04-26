use std::{collections::VecDeque, sync::Arc};

use anyhow::{Error, Result};
use iroh::endpoint::RecvStream;
use ps_buffer::Buffer;

use crate::{Interaction, Packet};

impl Interaction {
    pub async fn listen(
        recv_stream: Arc<tokio::sync::Mutex<RecvStream>>,
        packets: Arc<parking_lot::Mutex<VecDeque<Packet>>>,
        closed: Arc<parking_lot::RwLock<bool>>,
    ) -> Result<()> {
        let recv_stream_clone = recv_stream.clone();
        let result: Result<(), Error> = async move {
            loop {
                let mut guard = recv_stream_clone.lock().await;
                let mut length = [0u8; std::mem::size_of::<usize>()];

                guard.read_exact(&mut length[0..4]).await?;

                let length = usize::from_le_bytes(length) + 8;
                let mut buffer = Buffer::alloc_uninit(length)?;

                guard.read_exact(&mut buffer).await?;

                drop(guard);

                let packet = Packet::from_bytes(buffer)?;

                packets.lock().push_back(packet);
            }
        }
        .await;

        recv_stream.lock().await.stop(0u8.into())?;

        *closed.write() = true;

        result
    }
}
