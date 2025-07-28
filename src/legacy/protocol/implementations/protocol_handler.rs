use iroh::protocol::{AcceptError, ProtocolHandler};

use crate::{ScatterNet, ScatterNetProtocol, Terminate};

impl ProtocolHandler for ScatterNetProtocol {
    async fn accept(&self, connection: iroh::endpoint::Connection) -> Result<(), AcceptError> {
        let result = ScatterNet::init_peer(&self.net, connection, None);

        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(AcceptError::User {
                source: err.into_boxed_dyn_error(),
            }),
        }
    }

    async fn shutdown(&self) {
        self.net.terminate(0u8, &"ScatterNetProtocol is exiting.");
    }
}
