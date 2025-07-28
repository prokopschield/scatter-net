use std::fmt::Display;

use crate::PeerGroup;

impl Display for PeerGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("PeerGroup({})", self.read().config.name))
    }
}
