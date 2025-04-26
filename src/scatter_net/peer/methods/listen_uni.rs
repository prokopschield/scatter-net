use std::sync::Arc;

use anyhow::Result;

use crate::{spawn_and_forget, Interaction, Peer};

impl Peer {
    pub async fn listen_uni(peer: Arc<Self>) -> Result<()> {
        let connection = peer.connection.read().clone();

        loop {
            let stream = connection.accept_uni().await?;

            let interaction = Interaction::init(peer.clone(), stream, None);

            spawn_and_forget(async move {
                interaction.process().await?;
                Ok(())
            });
        }
    }
}
