use iroh::{endpoint::Connection, NodeId};

use crate::Peer;

impl Peer {
    pub fn replace_connection(
        &self,
        connection: Connection,
    ) -> Result<(), PeerReplaceConnectionError> {
        let conn_node_id = connection
            .remote_node_id()
            .map_err(PeerReplaceConnectionError::ReadingNodeIdFailed)?;

        let own_node_id = self.node_id();

        if conn_node_id != own_node_id {
            eprintln!("Tried to replace connection for {own_node_id} with {conn_node_id}!");

            return Err(PeerReplaceConnectionError::NodeIdMismatch {
                own_node_id,
                conn_node_id,
            });
        }

        *self.connection.write() = connection;

        eprintln!("Replaced connection for {}", self.node_id());

        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PeerReplaceConnectionError {
    #[error("Tried to replace connection to {own_node_id} with connection to {conn_node_id}.")]
    NodeIdMismatch {
        conn_node_id: NodeId,
        own_node_id: NodeId,
    },
    #[error(transparent)]
    ReadingNodeIdFailed(anyhow::Error),
}
