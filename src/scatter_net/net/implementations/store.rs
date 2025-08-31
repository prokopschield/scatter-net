use ps_datachunk::MbufDataChunk;
use ps_hkey::Store;

use crate::ScatterNet;

impl Store for ScatterNet {
    type Chunk<'c> = MbufDataChunk<'c>;
    type Error = ScatterNetStoreError;

    /// Attempts to get the `DataChunk` locally.
    fn get<'a>(&'a self, hash: &ps_hash::Hash) -> Result<Self::Chunk<'a>, Self::Error> {
        Ok(self.get_lake().get_encrypted_chunk(hash)?)
    }

    /// Fails if storing the chunk locally fails.
    fn put_encrypted<C: ps_datachunk::DataChunk>(&self, chunk: C) -> Result<(), Self::Error> {
        let chunk = chunk.into_owned();
        let result = self.get_lake().put_encrypted_chunk(&chunk);

        self.upsert_put(chunk);

        result?;

        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ScatterNetStoreError {
    #[error(transparent)]
    DataChunk(#[from] ps_datachunk::PsDataChunkError),
    #[error(transparent)]
    DataLake(#[from] ps_datalake::error::PsDataLakeError),
    #[error(transparent)]
    Hkey(#[from] ps_hkey::PsHkeyError),
}
