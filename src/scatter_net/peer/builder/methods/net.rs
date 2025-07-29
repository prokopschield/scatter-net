use crate::{PeerBuilder, ScatterNet};

impl PeerBuilder {
    #[must_use]
    pub fn net(mut self, net: ScatterNet) -> Self {
        self.net = Some(net);

        self
    }
}
