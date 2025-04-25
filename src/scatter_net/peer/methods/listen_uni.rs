use std::sync::Arc;

use anyhow::Result;

use crate::Peer;

impl Peer {
    pub async fn listen_uni(peer: Arc<Self>) -> Result<()> {
        let connection = peer.connection.read().clone();

        loop {
            let stream = connection.accept_uni().await?;

            let _todo = stream;
        }
    }
}
