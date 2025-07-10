mod implementations;
mod methods;

pub use implementations::*;
use iroh::SecretKey;
pub use methods::*;
use ps_datalake::lake::config::DataLakeConfig;
use serde::{Deserialize, Serialize};

use crate::PeerGroupConfig;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct NetConfig {
    pub lake_config: DataLakeConfig,
    pub peer_groups: Vec<PeerGroupConfig>,
    pub secret_key: Option<SecretKey>,
}
