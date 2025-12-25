use iroh::{endpoint::ConnectError, EndpointAddr};

use crate::{Peer, PeerBuilder, ALPN};

impl PeerBuilder {
    /// Initializes a connection to `node_addr`, consuming the builder.
    ///
    /// # Errors
    ///
    /// - [`ConnectError`] means the [`iroh`] connection failed.
    pub async fn connect_to(
        self,
        node_addr: impl Into<EndpointAddr>,
    ) -> Result<Peer, ConnectError> {
        let connection = self.net.endpoint.connect(node_addr, ALPN).await?;
        let peer = self.finalize(connection);

        Ok(peer)
    }
}
