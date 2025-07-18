#![allow(dead_code)]

#[macro_use]
extern crate dotenv_codegen;

mod routes;

use std::sync::Arc;

use axum::Router;
use backend::{db::Db, utils::log::init_tracing, State, WithDb};
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::trace::TraceLayer;
use tracing::info;

#[derive(Clone)]
struct AppState {
    db: Arc<Db>,
}

impl State for AppState {}

impl WithDb for AppState {
    fn db(&self) -> &Arc<Db> {
        &self.db
    }
}

#[tokio::main]
async fn main() {
    init_tracing();
    let listener = TcpListener::bind("0.0.0.0:3001").await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());

    let state = AppState {
        db: Arc::new(Db::new(dotenv!("DB_URI"), dotenv!("DB_NAME")).await),
    };

    let (_, web_router) = routes::Routes::make_router();

    let app = Router::new()
        .merge(web_router)
        .layer(CookieManagerLayer::new())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    axum::serve(listener, app).await.unwrap();
}
