use std::{future::Future, sync::Arc};

use ps_hkey::Hkey;

use crate::ScatterNet;

impl ScatterNet {
    pub fn put_raw<D: AsRef<[u8]>>(
        self: Arc<Self>,
        data: D,
    ) -> Result<ScatterNetPutRaw, ScatterNetPutRawError> {
        let data = data.as_ref();

        let hkey = self.lake.put_blob(data)?;

        // TODO: add to scatter queue

        Ok(ScatterNetPutRaw::Todo(hkey))
    }
}

pub enum ScatterNetPutRaw {
    Todo(Hkey),
}

impl Future for ScatterNetPutRaw {
    type Output = Result<Hkey, ScatterNetPutRawError>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let this = self.get_mut();

        let _ = cx; // TODO

        match this {
            Self::Todo(hkey) => std::task::Poll::Ready(Ok(hkey.clone())),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ScatterNetPutRawError {
    #[error(transparent)]
    Hkey(#[from] ps_hkey::PsHkeyError),
    #[error(transparent)]
    Lake(#[from] ps_datalake::error::PsDataLakeError),
}
