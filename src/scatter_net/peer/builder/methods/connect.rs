use std::mem::replace;

use iroh::endpoint::ConnectError;
use n0_future::Future;

use crate::PeerBuilder;

impl PeerBuilder {
    /// Initializes a connection
    ///
    /// # Errors
    ///
    /// Returns a [`ConnectError`] if the connection fails.
    pub fn connect(mut self) -> impl Future<Output = Result<Self, ConnectError>> {
        let node_id = self.node_addr.node_id;
        let node_addr = replace(&mut self.node_addr, node_id.into());

        self.connect_to(node_addr)
    }
}
