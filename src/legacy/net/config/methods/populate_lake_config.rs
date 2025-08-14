use std::{
    env,
    fs::{self, File},
    path::PathBuf,
    process,
};

use anyhow::Result;
use ps_datalake::lake::config::ConfigStoreEntry;

use crate::NetConfig;

impl NetConfig {
    pub fn populate_lake_config(&mut self) -> Result<()> {
        // Check if there's already a writable store
        let has_writable = self.lake_config.stores.iter().any(|entry| !entry.readonly);

        if !has_writable {
            // Create default writable store in user's home directory
            let home = env::var("HOME")
                .or_else(|_| env::var("USERPROFILE"))
                .unwrap_or_else(|_| ".".to_string());

            let pid = process::id();
            let lake_path = PathBuf::from(home)
                .join("scatter-net")
                .join(format!("{pid}.lake"));

            if let Some(lake_directory) = lake_path.parent() {
                // Create directories if they don't exist
                fs::create_dir_all(lake_directory).unwrap_or_else(|err| {
                    eprintln!("Failed to create lake directory: {err}");
                });

                if !fs::exists(&lake_path)? {
                    // Create the lake file
                    let lake_file = File::create(&lake_path).map_err(|err| {
                        eprintln!("Failed to create lake file: {err}");
                        err
                    })?;

                    // Truncate to 1 GiB
                    lake_file.set_len(1024 * 1024 * 1024).map_err(|err| {
                        eprintln!("Failed to set file size: {err}");
                        err
                    })?;
                }

                // Add the new store to the configuration
                self.lake_config.stores.push(ConfigStoreEntry {
                    filename: lake_path.to_string_lossy().into_owned(),
                    readonly: false,
                });

                // Add all existing entries in the directory as readonly
                if let Ok(entries) = fs::read_dir(lake_directory) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path != lake_path && path.extension().is_some_and(|ext| ext == "lake") {
                            self.lake_config.stores.push(ConfigStoreEntry {
                                filename: path.to_string_lossy().into_owned(),
                                readonly: true,
                            });
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
