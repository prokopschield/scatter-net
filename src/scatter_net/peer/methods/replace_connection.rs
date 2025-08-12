use iroh::endpoint::Connection;

use crate::Peer;

impl Peer {
    /// Replaces the connection to this [`Peer`], and re-runs `init`.
    pub fn replace_connection(&self, connection: Connection) {
        self.write().connection = connection;

        self.init();
    }
}
