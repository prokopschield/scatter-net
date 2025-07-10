use std::fmt::Debug;

use crate::ScatterNet;

impl Debug for ScatterNet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ScatterNet")
            .field("peers", &self.peers)
            .field("peer_groups", &self.peer_groups)
            .finish_non_exhaustive()
    }
}
