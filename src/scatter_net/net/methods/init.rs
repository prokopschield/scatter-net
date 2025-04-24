use std::sync::Arc;

use anyhow::Result;
use iroh::Endpoint;
use ps_datalake::lake::DataLake;
use tokio::spawn;

use crate::{spawn_and_forget, NetConfig, ScatterNet};

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

        Self::init_peer_groups(&net, net.config.peer_groups.clone())?;

        for peer_state in &net.config.peers {
            let net = net.clone();
            let peer_state = peer_state.clone();

            spawn_and_forget(async move {
                Self::connect_to(&net, peer_state.node_id, Some(peer_state)).await
            });
        }

        Ok(net)
    }
}
