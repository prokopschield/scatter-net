use std::{future::Future, mem::replace};

use ps_promise::Promise;

use crate::{scatter_net::peer::builder::methods::connect::{PeerBuilderConnect, PeerBuilderConnectError}, Peer, ALPN};

impl Future for PeerBuilderConnect {
    type Output = Result<Peer, PeerBuilderConnectError>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let mut this = self.get_mut();
        let mut value = replace(this, Self::Placeholder);

        match value {
            Self::Initial { builder } => {
                let connecting = Box::pin(builder.net.endpoint.connect_with_opts(builder.node_addr, ALPN, Default::default()));

                todo!()
            }
            Self::Connecting { builder, connecting } => {
                todo!()
            }
            Self::Placeholder => {
                todo!()
            }
        }
    }
}