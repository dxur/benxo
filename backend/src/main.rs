mod db;
mod events;
mod extractors;
mod middlewares;
mod models;
mod routes;
mod store;
mod utils;
mod validators;

#[macro_use]
extern crate dotenv_codegen;

use axum::{routing::get, Router};
use db::{product::Product, ModelInDb};
use events::EventBus;
use extractors::StoreId;
use routes::ApiRoutes;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::info;
use utils::router::RoutePacked;

#[derive(Clone)]
struct AppState {
    db: Arc<db::Db>,
    event_bus: Arc<events::EventBus>,
}

fn init_tracing() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();
}

#[tokio::main]
async fn main() {
    init_tracing();

    let db = db::Db::new(dotenv!("DB_URI"), dotenv!("DB_NAME")).await;
    Product::init_coll(&db).await.unwrap();

    let (tx, rx) = tokio::sync::mpsc::channel(10);
    let event_bus = events::EventBus::new(tx);

    let state = AppState {
        db: Arc::new(db),
        event_bus: Arc::new(event_bus),
    };

    EventBus::bind(state.clone(), rx);

    let app = Router::new()
        .nest_packed(ApiRoutes::make_router())
        .layer(TraceLayer::new_for_http())
        .route("/api/health", get(health))
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn health(StoreId(store_id): StoreId) -> String {
    format!("StoreId({})", store_id)
}
