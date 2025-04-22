use tokio::{sync::mpsc::*, task};
use tracing::info;

use crate::{models::product::ProductModelInDb, AppState};

type ObjectId = ();

#[derive(Debug)]
pub enum Event {
    ProductUpdated(ProductModelInDb),
    ProductDeleted(ProductModelInDb),
    ProductVarUpdated((ObjectId, String)),
    ProductVarDeleted((ObjectId, String)),
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
            Event::ProductUpdated(id) => on_product_updated(state, id).await,
            Event::ProductDeleted(id) => on_product_deleted(state, id).await,
            Event::ProductVarUpdated(var) => on_product_var_updated(state, var).await,
            Event::ProductVarDeleted(var) => on_product_var_deleted(state, var).await,
            Event::ThemeUpdated(id) => on_theme_updated(state, id).await,
        }
    }
}

pub async fn on_theme_updated(_: &AppState, id: ObjectId) {
    info!("Updating theme {:?}", id);
}

pub async fn on_product_updated(_: &AppState, product: ProductModelInDb) {
    info!("Updating product {:?}", product);
}

pub async fn on_product_deleted(_: &AppState, product: ProductModelInDb) {
    info!("Deleting product {:?}", product);
}

pub async fn on_product_var_updated(_: &AppState, var: (ObjectId, String)) {
    info!("Updating product Variant {:?}", var);
}

pub async fn on_product_var_deleted(_: &AppState, var: (ObjectId, String)) {
    info!("Deleting product Variant {:?}", var);
}
