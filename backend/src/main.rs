mod db;
mod events;
mod middlewares;
mod models;
mod routes;
mod utils;

#[macro_use]
extern crate dotenv_codegen;

use axum::Router;
use common::routes::*;
use events::EventBus;
use models::{product::ProductModel, Model};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::info;
use utils::router::RoutePacked;

#[derive(Clone)]
struct AppState {
    db: Arc<db::DB>,
    event_bus: Arc<events::EventBus>,
}

fn init_tracing() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
}

#[tokio::main]
async fn main() {
    init_tracing();

    let db = db::DB::new(dotenv!("DB_URI"), dotenv!("DB_NAME")).await;
    ProductModel::init_coll_in_db(&db).await.unwrap();

    let (tx, rx) = tokio::sync::mpsc::channel(10);
    let event_bus = events::EventBus::new(tx);

    let state = AppState {
        db: Arc::new(db),
        event_bus: Arc::new(event_bus),
    };

    EventBus::bind(state.clone(), rx);

    let app = Router::new()
        .nest_packed(ApiRoutes::make_router(
            ApiRoutes::get_all_products,
            ApiRoutes::get_one_product,
            ApiRoutes::create_product,
            ApiRoutes::update_product,
            ApiRoutes::delete_product,
        ))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
