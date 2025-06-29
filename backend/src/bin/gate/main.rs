#![allow(dead_code)]

#[macro_use]
extern crate dotenv_codegen;

mod routes;

use axum::Router;
use backend::db::Db;
use backend::utils::log::init_tracing;
use backend::utils::router::RoutePacked;
use backend::State;
use backend::WithDb;
use s3::creds::Credentials;
use s3::{Bucket, Region};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::trace::TraceLayer;
use tracing::info;

#[derive(Clone)]
struct AppState {
    db: Arc<Db>,
    bucket: Arc<Bucket>,
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
    let listener = TcpListener::bind("0.0.0.0:3002").await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());

    let region = Region::Custom {
        region: dotenv!("STORAGE_REGION").into(),
        endpoint: dotenv!("STORAGE_URI").into(),
    };

    let credentials = Credentials::new(
        Some(dotenv!("STORAGE_USER")),
        Some(dotenv!("STORAGE_PASSWORD")),
        None,
        None,
        None,
    )
    .unwrap();

    let bucket = Bucket::new(dotenv!("STORAGE_BUCKET_NAME"), region, credentials)
        .unwrap()
        .with_path_style();

    let state = AppState {
        db: Arc::new(Db::new(dotenv!("DB_URI"), dotenv!("DB_NAME")).await),
        bucket: Arc::new(*bucket),
    };

    let app = Router::new()
        .nest_packed(routes::Routes::make_router())
        .layer(CookieManagerLayer::new())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    axum::serve(listener, app).await.unwrap();
}
