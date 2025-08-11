use iroh::{endpoint::ConnectError, NodeAddr};

use crate::{PeerBuilder, ALPN};

impl PeerBuilder {
    /// Initializes a connection to `node_addr`, consuming the builder.
    ///
    /// # Errors
    ///
    /// Returns a [`ConnectError`] if the connection fails.
    pub async fn connect_to(
        mut self,
        node_addr: impl Into<NodeAddr>,
    ) -> Result<Self, ConnectError> {
        let connection = self.net.endpoint.connect(node_addr, ALPN).await?;

        self.connection = Some(connection);

        Ok(self)
    }
}
