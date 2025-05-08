use crate::models::product::ProductUpdate;
use crate::models::ObjectId;
use crate::store::StoreManager;
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
            Event::ProductCreated(_)
            | Event::ProductUpdated(_, _)
            | Event::ProductDeleted(_)
            | Event::ProductVarUpdated(_)
            | Event::ProductVarDeleted(_)
            | Event::ThemeUpdated(_) => on_updated(state).await,
        }
    }
}

pub async fn on_updated(state: &AppState) {
    let mut manager = match StoreManager::new().await {
        Ok(manager) => manager,
        Err(e) => {
            tracing::error!("Error creating StoreManager: {:?}", e);
            return;
        }
    };

    manager.full_update(state).await.unwrap();
}
