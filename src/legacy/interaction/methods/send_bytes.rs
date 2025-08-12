use bytes::Bytes;
use iroh::endpoint::{SendDatagramError, WriteError};

use crate::Interaction;

impl Interaction {
    pub async fn send_bytes(&self, bytes: Bytes) -> Result<(), InteractionSendBytesError> {
        self.send_stream
            .lock()
            .await
            .write_all(bytes.as_ref())
            .await
            .map_err(Into::into)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum InteractionSendBytesError {
    #[error("Failed to send datagram: {0}")]
    SendDatagramError(#[from] SendDatagramError),
    #[error("Failed to write data: {0}")]
    WriteError(#[from] WriteError),
}
