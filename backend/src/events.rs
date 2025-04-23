use common::models::ObjectId;
use tokio::{sync::mpsc::*, task};
use tracing::info;

use crate::{models::product::ProductModelInDb, AppState};

#[derive(Debug)]
pub enum Event {
    ProductCreated(ProductModelInDb),
    ProductUpdated(ProductModelInDb),
    ProductDeleted(ProductModelInDb),
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
            Event::ProductCreated(body) => on_product_created(state, body).await,
            Event::ProductUpdated(body) => on_product_updated(state, body).await,
            Event::ProductDeleted(body) => on_product_deleted(state, body).await,
            Event::ProductVarUpdated(body) => on_product_var_updated(state, body).await,
            Event::ProductVarDeleted(body) => on_product_var_deleted(state, body).await,
            Event::ThemeUpdated(body) => on_theme_updated(state, body).await,
        }
    }
}

pub async fn on_theme_updated(_: &AppState, id: ObjectId) {
    info!("Updating theme {:?}", id);
}

pub async fn on_product_updated(_: &AppState, product: ProductModelInDb) {
    info!("Updating product {:?}", product);
}

pub async fn on_product_created(_: &AppState, product: ProductModelInDb) {
    info!("Creating product {:?}", product);
}

pub async fn on_product_deleted(_: &AppState, product: ProductModelInDb) {
    info!("Deleting product {:?}", product);
}

pub async fn on_product_var_updated(_: &AppState, var: (String, ObjectId)) {
    info!("Updating product Variant {:?}", var);
}

pub async fn on_product_var_deleted(_: &AppState, var: (String, ObjectId)) {
    info!("Deleting product Variant {:?}", var);
}
