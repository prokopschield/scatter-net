use anyhow::Result;
use iroh::endpoint::Connection;

use crate::{
    legacy::peer::PeerState, Peer, PeerInnerReadonly, PeerInnerWritable, PeerUsage, ScatterNet,
};

impl Peer {
    pub fn init(net: ScatterNet, connection: Connection, state: Option<PeerState>) -> Result<Self> {
        let node_id = connection.remote_node_id()?;

        let mut state = state.unwrap_or_else(|| PeerState {
            node_id,
            terminated: false,
            usage: PeerUsage::default(),
        });

        state.terminated = false;

        let peer = Self::from_inner(
            PeerInnerReadonly { net, node_id },
            PeerInnerWritable { connection, state },
        );

        peer.clone().listen();

        Ok(peer)
    }
}
