#![allow(dead_code)]

mod db;
mod events;
mod extractors;
mod middlewares;
mod models;
mod routes;
mod store;
mod utils;
mod validators;

use axum::Router;
use db::ModelInitFn;
use routes::api::ApiRoutes;
use routes::web::WebRoutes;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::trace::TraceLayer;
use tracing::info;
use utils::log::init_tracing;
use utils::router::RoutePacked;

#[macro_use]
extern crate dotenv_codegen;

#[derive(Clone)]
struct AppState {
    db: Arc<db::Db>,
}

#[tokio::main]
async fn main() {
    init_tracing();

    let db = db::Db::new(dotenv!("DB_URI"), dotenv!("DB_NAME")).await;

    let state = AppState { db: Arc::new(db) };

    let (_, web_router) = WebRoutes::make_router();
    let api_router = ApiRoutes::make_router();

    let app = Router::new()
        .merge(web_router)
        .nest_packed(api_router)
        .layer(CookieManagerLayer::new())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
