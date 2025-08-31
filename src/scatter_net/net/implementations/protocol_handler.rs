use iroh::protocol::{AcceptError, ProtocolHandler};

use crate::ScatterNet;

impl ProtocolHandler for ScatterNet {
    async fn accept(
        &self,
        connection: iroh::endpoint::Connection,
    ) -> Result<(), iroh::protocol::AcceptError> {
        self.accept_connection(connection)
            .map_err(AcceptError::from_err)
            .and(Ok(()))
    }
}
