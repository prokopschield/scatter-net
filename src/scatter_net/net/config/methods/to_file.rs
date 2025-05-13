use std::{fs::File, io::Write};

use crate::NetConfig;

impl NetConfig {
    pub fn to_file(&self, path: &str) -> std::io::Result<()> {
        File::create(path)?.write_all(self.to_string().as_bytes())
    }
}
