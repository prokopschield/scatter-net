use iroh::{endpoint::ConnectError, EndpointAddr};
use n0_future::Future;

use crate::{Peer, PeerBuilder};

impl PeerBuilder {
    /// Initializes a connection, consuming the builder.
    ///
    /// # Errors
    ///
    /// - [`ConnectError`] means the [`iroh`] connection failed.
    pub fn connect(self) -> impl Future<Output = Result<Peer, ConnectError>> {
        let node_addr = EndpointAddr {
            addrs: self.direct_addresses.iter().cloned().collect(),
            id: self.node_id,
        };

        self.connect_to(node_addr)
    }
}
