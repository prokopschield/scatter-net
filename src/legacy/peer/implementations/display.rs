use std::fmt::Display;

use crate::Peer;

impl Display for Peer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.state.read().node_id))
    }
}
