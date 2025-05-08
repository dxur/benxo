mod config;
mod registry;

use crate::models::product::{ProductPublic, ProductUpdate};
use crate::utils::error::Error;
use crate::utils::types::{AtLeast, Result};
use crate::AppState;
use registry::{Registry, Templates};
use tokio::fs;

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
