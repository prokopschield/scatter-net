use std::sync::Arc;

use ps_datalake::lake::DataLake;

use crate::ScatterNet;

impl ScatterNet {
    #[must_use]
    pub fn get_lake(&self) -> Arc<DataLake> {
        self.lake.clone()
    }
}
