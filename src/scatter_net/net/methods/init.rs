use std::sync::Arc;

use anyhow::Result;
use iroh::Endpoint;
use ps_datalake::lake::DataLake;
use tokio::spawn;

use crate::{NetConfig, ScatterNet};

impl ScatterNet {
    /// Initializes a [`ScatterNet`] instance.
    /// # Errors
    /// An Error is returned if binding the socket fails.
    pub async fn init(config: NetConfig) -> Result<Arc<Self>> {
        let mut builder = Endpoint::builder()
            .discovery_dht()
            .discovery_local_network()
            .discovery_n0();

        if let Some(secret_key) = config.secret_key.clone() {
            builder = builder.secret_key(secret_key);
        }

        let endpoint = builder.bind().await?;
        let node_id = endpoint.node_id();

        eprintln!("Initialized node {node_id}");

        let net = Self {
            config: config.clone(),
            endpoint,
            lake: Arc::new(DataLake::init(config.lake_config)?),
            node_id,
            peer_groups: Arc::default(),
            peers: Arc::default(),
            state: Arc::default(),
        };

        let net = Arc::new(net);

        spawn(Self::accept_loop(net.clone()));

        Ok(net)
    }
}
