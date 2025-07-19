use anyhow::Result;

use crate::{spawn_and_forget, Interaction, Peer};

impl Peer {
    pub async fn listen_bi(self) -> Result<()> {
        let connection = self.read().connection.clone();

        loop {
            let channel = connection.accept_bi().await?;

            let interaction = Interaction::init(self.clone(), channel.1, Some(channel.0));

            spawn_and_forget(async move {
                interaction.process().await?;
                Ok(())
            });
        }
    }
}
