use crate::models::product::ProductUpdate;
use crate::models::ObjectId;
use crate::store::StoreManager;
use crate::utils::types::AtLeast;
use tokio::{sync::mpsc::*, task};
use tracing::info;

use crate::db::product::ProductInDb;
use crate::AppState;

#[derive(Debug)]
pub enum Event {
    ProductCreated(ProductInDb),
    ProductUpdated(ProductUpdate, ProductInDb),
    ProductDeleted(ProductInDb),
    ProductVarUpdated((String, ObjectId)), // (sku, product_id)
    ProductVarDeleted((String, ObjectId)), // (sku, product_id)
    ThemeUpdated(ObjectId),
}

pub struct EventBus {
    sender: Sender<Event>,
}

impl EventBus {
    pub fn new(sender: Sender<Event>) -> Self {
        EventBus { sender }
    }

    pub fn bind(state: AppState, mut receiver: Receiver<Event>) {
        task::spawn(async move {
            while let Some(event) = receiver.recv().await {
                Self::handle_event(&state, event).await;
            }
        });
    }

    pub async fn emit(&self, event: Event) {
        if let Err(e) = self.sender.send(event).await {
            eprintln!("Failed to emit event: {}", e);
        }
    }

    async fn handle_event(state: &AppState, event: Event) {
        match event {
            Event::ProductCreated(value) => on_product_updated(state, value, None).await,
            Event::ProductUpdated(update, value) => {
                on_product_updated(state, value, Some(update)).await
            }
            Event::ProductDeleted(value) => on_product_deleted(state, value).await,
            Event::ProductVarUpdated(value) => on_product_var_updated(state, value).await,
            Event::ProductVarDeleted(value) => on_product_var_deleted(state, value).await,
            Event::ThemeUpdated(value) => on_theme_updated(state, value).await,
        }
    }
}

pub async fn on_theme_updated(_: &AppState, id: ObjectId) {
    info!("Updating theme {:?}", id);
}

pub async fn on_product_updated(
    state: &AppState,
    product: ProductInDb,
    update: Option<ProductUpdate>,
) {
    let mut manager = match StoreManager::new().await {
        Ok(manager) => manager,
        Err(e) => {
            tracing::error!("Error creating StoreManager: {:?}", e);
            return;
        }
    };

    let body = match update {
        Some(u) => AtLeast::All(u, product.into()),
        None => AtLeast::Second(product.into()),
    };

    manager.product_updated(state, body).await.unwrap();
}

pub async fn on_product_deleted(state: &AppState, product: ProductInDb) {
    let mut manager = match StoreManager::new().await {
        Ok(manager) => manager,
        Err(e) => {
            tracing::error!("Error creating StoreManager: {:?}", e);
            return;
        }
    };

    manager
        .product_deleted(state, product.into())
        .await
        .unwrap();
}

pub async fn on_product_var_updated(_: &AppState, var: (String, ObjectId)) {
    info!("Updating product Variant {:?}", var);
}

pub async fn on_product_var_deleted(_: &AppState, var: (String, ObjectId)) {
    info!("Deleting product Variant {:?}", var);
}
