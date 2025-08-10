use crate::{Peer, PeerBuilder, PeerInnerReadonly, PeerInnerWritable, PeerState, PeerUsage, ALPN};

impl PeerBuilder {
    /// Finish the building process and connect to the Peer
    ///
    /// # Errors
    ///
    /// - [`PeerBuilderConnectError::Connect`] means the Iroh connection failed.
    /// - [`PeerBuilderConnectError::SelectPeerGroup`] means the peer couldn't be placed into a `PeerGroup`.
    pub async fn connect(self) -> Result<Peer, PeerBuilderConnectError> {
        let Self {
            connection,
            net,
            node_addr,
            peer_group,
            state,
        } = self;

        let node_id = node_addr.node_id;

        let connection = match connection {
            Some(connection) => connection,
            None => net.endpoint.connect(node_addr, ALPN).await?,
        };

        let peer = Peer::from_inner(
            PeerInnerReadonly { net, node_id },
            PeerInnerWritable {
                connection,
                state: state.unwrap_or_else(|| PeerState {
                    node_id,
                    terminated: false,
                    usage: PeerUsage::default(),
                }),
            },
        );

        if let Some(peer_group) = peer_group {
            peer_group.insert_peer(peer.clone());
        } else {
            peer.clone().select_peer_group().await?;
        }

        Ok(peer)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PeerBuilderConnectError {
    #[error(transparent)]
    Connect(#[from] iroh::endpoint::ConnectError),
    #[error(transparent)]
    SelectPeerGroup(#[from] crate::PeerSelectPeerGroupError),
}
