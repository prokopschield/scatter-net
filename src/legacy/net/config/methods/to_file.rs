use std::{fs::File, io::Write, path::Path};

use crate::NetConfig;

impl NetConfig {
    pub fn to_file<P>(&self, path: P) -> std::io::Result<()>
    where
        P: AsRef<Path>,
    {
        File::create(path)?.write_all(self.to_string().as_bytes())
    }
}
