#[cfg(target_arch = "wasm32")]
compile_error!("The binary should not be compiled for `wasm32`.");

#[cfg(not(feature = "server"))]
compile_error!("Should be compiled with the `server` feature enabled.");

mod api;
mod db;
mod events;
mod middlewares;
mod models;
mod routes;
mod utils;
mod validators;

#[macro_use]
extern crate dotenv_codegen;

use crate::api::*;
use axum::Router;
use db::{product::Product, ModelInDb};
use events::EventBus;
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
        .nest_packed(ApiRoutes::make_router(
            ApiRoutes::get_all_products,
            ApiRoutes::get_one_product,
            ApiRoutes::create_product,
            ApiRoutes::update_product,
            ApiRoutes::delete_product,
            ApiRoutes::get_some_variants,
            ApiRoutes::get_one_variant,
            ApiRoutes::create_variant,
            ApiRoutes::update_variant,
            ApiRoutes::delete_variant,
            ApiRoutes::get_all_users,
            ApiRoutes::get_one_user,
            ApiRoutes::create_user,
            ApiRoutes::update_user,
            ApiRoutes::delete_user,
        ))
        // .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
