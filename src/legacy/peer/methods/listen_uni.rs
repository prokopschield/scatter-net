use anyhow::Result;

use crate::{spawn_and_forget, Interaction, Peer};

impl Peer {
    pub async fn listen_uni(self) -> Result<()> {
        let connection = self.read().connection.clone();

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
