use iroh::SecretKey;
use ps_datalake::lake::config::DataLakeConfig;
use serde::{Deserialize, Serialize};

use crate::PeerGroupConfig;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct NetConfig {
    pub lake: DataLakeConfig,
    pub peer_groups: Vec<PeerGroupConfig>,
    pub secret_key: Option<SecretKey>,
}
