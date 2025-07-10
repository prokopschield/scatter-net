use iroh::protocol::ProtocolHandler;

use crate::{ScatterNet, ScatterNetProtocol, Terminate};

impl ProtocolHandler for ScatterNetProtocol {
    fn accept(
        &self,
        connection: iroh::endpoint::Connection,
    ) -> n0_future::future::Boxed<anyhow::Result<()>> {
        let result = ScatterNet::init_peer(&self.net, connection, None);

        Box::pin(async { result.map(|_| ()) })
    }

    fn shutdown(&self) -> n0_future::future::Boxed<()> {
        self.net.terminate(0u8, &"ScatterNetProtocol is exiting.");

        Box::pin(async {})
    }
}
