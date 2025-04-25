use std::sync::Arc;

use anyhow::Result;

use crate::Peer;

impl Peer {
    pub async fn listen_bi(peer: Arc<Self>) -> Result<()> {
        let connection = peer.connection.read().clone();

        loop {
            let channel = connection.accept_bi().await?;

            let _todo = channel;
        }
    }
}
