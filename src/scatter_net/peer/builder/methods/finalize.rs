use iroh::endpoint::Connection;

use crate::{Peer, PeerBuilder, PeerInnerReadonly, PeerInnerWritable, PeerState, PeerUsage};

impl PeerBuilder {
    /// Finish the building process and connect to the Peer
    ///
    /// # Errors
    ///
    /// - [`PeerBuilderFinalizeError::SelectPeerGroup`] means the peer couldn't be placed into a `PeerGroup`.
    pub fn finalize(self, connection: Connection) -> Result<Peer, PeerBuilderFinalizeError> {
        let Self {
            direct_addresses: _,
            net,
            node_id,
            relay_url: _,
            peer_group,
            state,
        } = self;

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
        }

        peer.init();

        Ok(peer)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PeerBuilderFinalizeError {
    #[error(transparent)]
    SelectPeerGroup(#[from] crate::PeerSelectPeerGroupError),
}
