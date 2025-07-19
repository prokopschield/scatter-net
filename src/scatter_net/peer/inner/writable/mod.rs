use iroh::endpoint::Connection;

use crate::PeerState;

#[derive(Clone, Debug)]
pub struct PeerInnerWritable {
    pub connection: Connection,
    pub state: PeerState,
}
