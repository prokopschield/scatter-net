#![allow(clippy::missing_errors_doc)]

use anyhow::Result;
use ps_datalake::lake::config::{ConfigStoreEntry, DataLakeConfig};
use scatter_net::{NetConfig, ScatterNet};
use tokio::fs::OpenOptions;

#[tokio::main]
async fn main() -> Result<()> {
    let filename = "store.dat".to_string();

    truncate(&filename).await?;

    let lake_config = DataLakeConfig {
        store: vec![ConfigStoreEntry {
            filename,
            readonly: false,
        }],
    };

    let config = NetConfig {
        lake_config,
        ..Default::default()
    };

    let net = ScatterNet::init(config).await?;

    eprintln!("Press CTRL+C to exit!");

    tokio::signal::ctrl_c().await?;

    eprintln!("\nExiting...");

    ScatterNet::terminate(&net, 0u8, &"SIGINT");

    eprintln!("Terminated successfully.");

    Ok(())
}

pub async fn truncate(filename: &str) -> Result<()> {
    const ONE_MB: u64 = 1024 * 1024;

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(filename)
        .await?;

    file.set_len(ONE_MB).await?;

    Ok(())
}
