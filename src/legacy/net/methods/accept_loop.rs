use crate::{spawn_and_forget, ScatterNet};

impl ScatterNet {
    pub async fn accept_loop(self) {
        loop {
            if let Some(incoming) = self.endpoint.accept().await {
                spawn_and_forget(Self::handle_incoming_connection(self.clone(), incoming));
            }
        }
    }
}
