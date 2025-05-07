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

    pub async fn index_updated(&mut self, state: &AppState) -> Result<()> {
        let template = self.registry.get_template(state, Templates::INDEX).await?;
        let globals = liquid::object!({
            // settings latter
        });

        fs::write(
            "dist/index.html",
            template
                .render(&globals)
                .map_err(|e| Error::from(e.to_string()))?,
        )
        .await
        .map_err(|e| Error::from(e.to_string()))?;

        Ok(())
    }

    pub async fn product_updated(
        &mut self,
        state: &AppState,
        body: AtLeast<ProductUpdate, ProductPublic>,
    ) -> Result<()> {
        let (update, product) = match body {
            AtLeast::First(u) => (Some(u), None),
            AtLeast::Second(p) => (None, Some(p)),
            AtLeast::All(u, p) => (Some(u), Some(p)),
        };

        // TODO: if slug get updated or product

        if product.is_none() || update.as_ref().map_or(false, |v| v.body.slug.is_some()) {
            // TODO: delete the product
        }

        if let Some(prod) = product {
            let template = self
                .registry
                .get_template(state, Templates::PRODUCT)
                .await?;
            let globals = liquid::object!({
                "product": liquid::to_object(&prod).unwrap(),
            });

            fs::write(
                format!("dist/{}.html", prod.slug),
                template
                    .render(&globals)
                    .map_err(|e| Error::from(e.to_string()))?,
            )
            .await
            .map_err(|e| Error::from(e.to_string()))?;
        }
        Ok(())
    }

    pub async fn product_deleted(&mut self, _: &AppState, _: ProductPublic) -> Result<()> {
        // TODO: delete the product
        Ok(())
    }
}
