use std::sync::Arc;

use anyhow::Result;

use crate::Peer;

impl Peer {
    pub async fn listen_dg(peer: Arc<Self>) -> Result<()> {
        let connection = peer.connection.read().clone();

        loop {
            let dg = connection.read_datagram().await?;

            let _todo = dg;
        }
    }
}
