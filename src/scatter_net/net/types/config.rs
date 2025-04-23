use iroh::SecretKey;
use ps_datalake::lake::config::DataLakeConfig;

#[derive(Clone, Debug, Default)]
pub struct NetConfig {
    pub lake_config: DataLakeConfig,
    pub secret_key: Option<SecretKey>,
}
