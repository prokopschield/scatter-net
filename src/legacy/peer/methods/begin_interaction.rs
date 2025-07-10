use std::sync::Arc;

use iroh::endpoint::ConnectionError;

use crate::{Interaction, Peer};

impl Peer {
    pub async fn begin_interaction(
        self: Arc<Self>,
    ) -> Result<Interaction, PeerBeginInteractionError> {
        let connection = self.connection.read().clone();
        let channel = connection.open_bi().await?;
        let interaction = Interaction::init(self, channel.1, Some(channel.0));

        Ok(interaction)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PeerBeginInteractionError {
    #[error(transparent)]
    Connection(#[from] ConnectionError),
}
