use std::sync::Arc;

use anyhow::Result;

use crate::{spawn_and_forget, Interaction, Peer};

impl Peer {
    pub async fn listen_uni(self: Arc<Self>) -> Result<()> {
        let connection = self.connection.read().clone();

        loop {
            let stream = connection.accept_uni().await?;

            let interaction = Interaction::init(self.clone(), stream, None);

            spawn_and_forget(async move {
                interaction.process().await?;
                Ok(())
            });
        }
    }
}
