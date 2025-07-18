use ps_datalake::lake::DataLake;

use crate::ScatterNet;

impl ScatterNet {
    #[must_use]
    pub fn get_lake(&self) -> &DataLake {
        &self.lake
    }
}
