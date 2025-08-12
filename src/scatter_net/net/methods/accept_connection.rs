use iroh::endpoint::Connection;

use crate::{Peer, ScatterNet};

impl ScatterNet {
    /// Accepts the [`Connection`] and returns the [`Peer`].
    ///
    /// # Errors
    /// - [`ScatterNetAcceptConnectionError::RemoteNodeId`] means the [`Connection`]'s `NodeId` couldn't be resolved.
    pub fn accept_connection(
        &self,
        connection: Connection,
    ) -> Result<Peer, ScatterNetAcceptConnectionError> {
        let node_id = connection.remote_node_id()?;
        let peer = self.read().peers.get(&node_id).cloned();

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
    RemoteNodeId(#[from] iroh::endpoint::RemoteNodeIdError),
}
