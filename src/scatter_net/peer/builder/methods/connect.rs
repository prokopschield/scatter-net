use iroh::NodeAddr;
use n0_future::Future;

use crate::{Peer, PeerBuilder, PeerBuilderConnectToError};

impl PeerBuilder {
    /// Initializes a connection, consuming the builder.
    ///
    /// # Errors
    ///
    /// - [`PeerBuilderConnectToError::Connect`] means the [`iroh`] connection failed.
    /// - [`PeerBuilderConnectToError::Finalize`] means the [`Peer`] initialization failed.
    pub fn connect(self) -> impl Future<Output = Result<Peer, PeerBuilderConnectToError>> {
        let node_addr = NodeAddr {
            direct_addresses: self.direct_addresses.iter().copied().collect(),
            node_id: self.node_id,
            relay_url: self.relay_url.clone(),
        };

        self.connect_to(node_addr)
    }
}
