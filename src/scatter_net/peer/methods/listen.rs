use iroh::endpoint::Connection;

use crate::{spawn_and_forget, Interaction, Peer};

impl Peer {
    pub fn listen(self, connection: Connection) {
        spawn_and_forget::<_, ()>(async move {
            loop {
                let channel = connection.accept_bi().await?;
                let interaction = Interaction::init(self.clone(), channel.1, Some(channel.0));

                spawn_and_forget(async move { Ok(interaction.process().await?) });
            }
        });
    }
}
