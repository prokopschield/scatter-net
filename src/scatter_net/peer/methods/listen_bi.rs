use std::sync::Arc;

use anyhow::Result;

use crate::{spawn_and_forget, Interaction, Peer};

impl Peer {
    pub async fn listen_bi(peer: Arc<Self>) -> Result<()> {
        let connection = peer.connection.read().clone();

        loop {
            let channel = connection.accept_bi().await?;

            let interaction = Interaction::init(peer.clone(), channel.1, Some(channel.0));

            spawn_and_forget(async move {
                interaction.process().await?;
                Ok(())
            });
        }
    }
}
