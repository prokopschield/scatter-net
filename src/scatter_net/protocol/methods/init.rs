use anyhow::Result;

use crate::{NetConfig, NetState, ScatterNet, ScatterNetProtocol};

impl ScatterNetProtocol {
    pub async fn init(config: NetConfig, state: NetState) -> Result<Self> {
        let net = ScatterNet::init(config, state).await?;

        Ok(Self { net })
    }
}
