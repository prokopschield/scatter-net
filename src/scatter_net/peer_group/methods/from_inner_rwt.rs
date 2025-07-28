use ps_rwt::RWT;

use crate::{
    scatter_net::peer_group::inner::{PeerGroupInnerReadonly, PeerGroupInnerWritable},
    PeerGroup,
};

impl PeerGroup {
    #[must_use]
    pub const fn from_inner_rwt(
        inner: RWT<PeerGroupInnerReadonly, PeerGroupInnerWritable>,
    ) -> Self {
        Self { inner }
    }
}
