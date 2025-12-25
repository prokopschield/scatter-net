use iroh::protocol::{AcceptError, ProtocolHandler};
use n0_error::Meta;

use crate::{ScatterNet, ScatterNetProtocol, Terminate};

impl ProtocolHandler for ScatterNetProtocol {
    async fn accept(&self, connection: iroh::endpoint::Connection) -> Result<(), AcceptError> {
        let result = ScatterNet::init_peer(&self.net, connection, None);

        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(AcceptError::User {
                meta: Meta::new(),
                source: err.into_boxed_dyn_error().into(),
            }),
        }
    }

    async fn shutdown(&self) {
        self.net.terminate(0u8, &"ScatterNetProtocol is exiting.");
    }
}
