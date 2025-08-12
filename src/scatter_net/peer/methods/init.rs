use crate::{spawn_and_forget, Peer};

impl Peer {
    pub fn init(&self) {
        self.clone().listen(self.read().connection.clone());

        if !self
            .net
            .get_peer_groups()
            .iter()
            .any(|peer_group| peer_group.has_peer(self))
        {
            let peer = self.clone();

            spawn_and_forget(async move { Ok(peer.select_peer_group().await?) });
        }

        let node_id = self.node_id;

        self.net.write().peers.insert(node_id, self.clone());
    }
}
