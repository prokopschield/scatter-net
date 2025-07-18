use std::fmt::Debug;

use crate::ScatterNet;

impl Debug for ScatterNet {
    #[allow(clippy::significant_drop_tightening)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let guard = self.write();

        let peers = &guard.peers;
        let peer_groups = &guard.peer_groups;

        f.debug_struct("ScatterNet")
            .field("peers", peers)
            .field("peer_groups", peer_groups)
            .finish_non_exhaustive()
    }
}
