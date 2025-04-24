use std::sync::Arc;

use anyhow::Result;
use iroh::Endpoint;
use parking_lot::lock_api::RwLock;
use ps_datalake::lake::DataLake;
use tokio::spawn;

use crate::{spawn_and_forget, NetConfig, NetState, ScatterNet, ALPN};

impl ScatterNet {
    /// Initializes a [`ScatterNet`] instance.
    /// # Errors
    /// An Error is returned if binding the socket fails.
    pub async fn init(config: NetConfig, state: NetState) -> Result<Arc<Self>> {
        let mut builder = Endpoint::builder()
            .alpns(vec![ALPN.to_vec()])
            .discovery_dht()
            .discovery_local_network()
            .discovery_n0();

        if let Some(secret_key) = config.secret_key.clone() {
            builder = builder.secret_key(secret_key);
        }

        let endpoint = builder.bind().await?;
        let node_id = endpoint.node_id();

        let peers_state = state.peers.clone();

        eprintln!("Initialized node {node_id}");

        let net = Self {
            config: config.clone(),
            endpoint,
            lake: Arc::new(DataLake::init(config.lake_config)?),
            node_id,
            peer_groups: Arc::default(),
            peers: Arc::default(),
            state: Arc::from(RwLock::new(state)),
        };

        let net = Arc::new(net);

        spawn(Self::accept_loop(net.clone()));

        Self::init_peer_groups(&net, net.config.peer_groups.clone())?;

        for peer_state in peers_state {
            let net = net.clone();

            spawn_and_forget(async move {
                Self::connect_to(&net, peer_state.node_id, Some(peer_state)).await
            });
        }

        Ok(net)
    }
}
