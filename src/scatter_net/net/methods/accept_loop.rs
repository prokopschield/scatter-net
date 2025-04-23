use std::sync::Arc;

use tokio::spawn;

use crate::ScatterNet;

impl ScatterNet {
    pub async fn accept_loop(net: Arc<Self>) {
        loop {
            if let Some(incoming) = net.endpoint.accept().await {
                spawn(Self::handle_incoming_connection(net.clone(), incoming));
            }
        }
    }
}
