use iroh::endpoint::Connection;

use crate::{Peer, ScatterNet};

impl ScatterNet {
    /// Accepts the [`Connection`] and returns the [`Peer`].
    ///
    /// # Errors
    /// - [`ScatterNetAcceptConnectionError::RemoteEndpointId`] means the [`Connection`]'s `EndpointId` couldn't be resolved.
    pub fn accept_connection(
        &self,
        connection: Connection,
    ) -> Result<Peer, ScatterNetAcceptConnectionError> {
        let node_id = connection.remote_id();
        let peer = self.get_peer(&node_id);

        if let Some(peer) = peer {
            peer.replace_connection(connection);

            return Ok(peer);
        }

        let peer = Peer::builder(self.clone(), node_id).finalize(connection);

        Ok(peer)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ScatterNetAcceptConnectionError {
    #[error(transparent)]
    RemoteEndpointId(#[from] iroh::endpoint::RemoteEndpointIdError),
}
