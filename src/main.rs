#![allow(clippy::missing_errors_doc)]

use std::io::SeekFrom;

use anyhow::Result;
use ps_datalake::lake::config::{ConfigStoreEntry, DataLakeConfig};
use scatter_net::{NetConfig, ScatterNet};
use tokio::{
    fs::OpenOptions,
    io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt},
};

#[tokio::main]
async fn main() -> Result<()> {
    let state_file = "state.json";
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

    let mut state = Vec::new();
    OpenOptions::new()
        .read(true)
        .open(&state_file)
        .await?
        .read_to_end(&mut state)
        .await?;

    let state = serde_json::de::from_slice(&state)?;

    let net = ScatterNet::init(config, state).await?;

    eprintln!("Press CTRL+C to exit!");

    tokio::signal::ctrl_c().await?;

    save_state(&net, state_file).await?;

    eprintln!("\nExiting...");

    ScatterNet::terminate(&net, 0u8, &"SIGINT");

    eprintln!("Terminated successfully.");

    Ok(())
}

pub async fn save_state(net: &ScatterNet, path: &str) -> Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(path)
        .await?;

    let mut current_state = Vec::new();

    file.read_to_end(&mut current_state).await?;

    let mut new_state = serde_json::ser::to_string_pretty(&net.get_state()?)?;

    new_state.push('\n');

    if current_state.as_slice() != new_state.as_bytes() {
        file.set_len(0).await?;
        file.seek(SeekFrom::Start(0)).await?;
        file.write_all(new_state.as_bytes()).await?;
    }

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
