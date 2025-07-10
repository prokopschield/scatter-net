use std::{
    fs::{exists, File},
    io::Read,
    path::Path,
};

use crate::NetConfig;

impl NetConfig {
    /// Attempts to read a file and deserialize it into a `NetConfig`.
    pub fn from_file<P>(filename: P) -> Result<Self, NetConfigFromFileError>
    where
        P: AsRef<Path>,
    {
        let filename = filename.as_ref();

        if !exists(filename)? {
            return Ok(Self::default());
        }

        let mut file = File::open(filename)?;
        let mut contents = String::new();

        file.read_to_string(&mut contents)?;

        Ok(contents.parse()?)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum NetConfigFromFileError {
    #[error(transparent)]
    FromStr(#[from] crate::NetConfigFromStrError),
    #[error(transparent)]
    IO(#[from] std::io::Error),
}
