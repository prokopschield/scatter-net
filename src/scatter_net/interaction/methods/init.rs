use std::sync::Arc;

use iroh::endpoint::{RecvStream, SendStream};
use tokio::sync::Mutex;

use crate::{Interaction, Peer};

impl Interaction {
    pub fn init(
        peer: Arc<Peer>,
        recv_stream: Option<RecvStream>,
        send_stream: Option<SendStream>,
    ) -> Arc<Self> {
        let interaction = Self {
            peer,
            recv_stream: recv_stream.map(Mutex::new).map(Arc::new),
            send_stream: send_stream.map(Mutex::new).map(Arc::new),
            packets: Arc::default(),
        };

        let interaction = Arc::new(interaction);

        crate::spawn_and_forget(Self::listen(interaction.clone()));

        interaction
    }
}
