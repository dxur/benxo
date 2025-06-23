mod config;
mod registry;

use crate::utils::types::Result;
use crate::AppState;
use registry::Registry;

pub struct StoreManager {
    registry: Registry,
}

impl StoreManager {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            registry: Registry::hardcoded().await?,
        })
    }

    pub async fn full_update(&mut self, _: &AppState) -> Result<()> {
        Ok(())
    }
}
