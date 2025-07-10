use std::fmt::Display;

use crate::NetConfig;

impl Display for NetConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&toml::ser::to_string_pretty(&self).unwrap_or_default())
    }
}
