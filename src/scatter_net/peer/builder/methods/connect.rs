use iroh::{endpoint::ConnectError, NodeAddr};
use n0_future::Future;

use crate::{Peer, PeerBuilder};

impl PeerBuilder {
    /// Initializes a connection, consuming the builder.
    ///
    /// # Errors
    ///
    /// - [`ConnectError`] means the [`iroh`] connection failed.
    pub fn connect(self) -> impl Future<Output = Result<Peer, ConnectError>> {
        let node_addr = NodeAddr {
            direct_addresses: self.direct_addresses.iter().copied().collect(),
            node_id: self.node_id,
            relay_url: self.relay_url.clone(),
        };

        self.connect_to(node_addr)
    }
}
