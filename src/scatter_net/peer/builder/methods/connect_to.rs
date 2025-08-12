use iroh::NodeAddr;

use crate::{Peer, PeerBuilder, ALPN};

impl PeerBuilder {
    /// Initializes a connection to `node_addr`, consuming the builder.
    ///
    /// # Errors
    ///
    /// - [`PeerBuilderConnectToError::Connect`] means the [`iroh`] connection failed.
    /// - [`PeerBuilderConnectToError::Finalize`] means the [`Peer`] initialization failed.
    pub async fn connect_to(
        self,
        node_addr: impl Into<NodeAddr>,
    ) -> Result<Peer, PeerBuilderConnectToError> {
        let connection = self.net.endpoint.connect(node_addr, ALPN).await?;
        let peer = self.finalize(connection).await?;

        Ok(peer)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PeerBuilderConnectToError {
    #[error(transparent)]
    Connect(#[from] iroh::endpoint::ConnectError),
    #[error(transparent)]
    Finalize(#[from] crate::PeerBuilderFinalizeError),
}
