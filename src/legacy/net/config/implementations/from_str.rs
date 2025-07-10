use std::str::FromStr;

use crate::NetConfig;

impl FromStr for NetConfig {
    type Err = NetConfigFromStrError;

    /// Attempts to deserialize a `NetConfig`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Ok(Self::default());
        }

        let first_char = s.as_bytes()[0];

        let config: Self = if first_char == b'{' {
            serde_json::from_str(s)?
        } else {
            toml::from_str(s)?
        };

        Ok(config)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum NetConfigFromStrError {
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Toml(#[from] toml::de::Error),
}
