use std::sync::Arc;

use crate::{spawn_and_forget, ScatterNet};

impl ScatterNet {
    pub async fn accept_loop(net: Arc<Self>) {
        loop {
            if let Some(incoming) = net.endpoint.accept().await {
                spawn_and_forget(Self::handle_incoming_connection(net.clone(), incoming));
            }
        }
    }
}
