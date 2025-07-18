#![allow(dead_code)]

#[macro_use]
extern crate dotenv_codegen;

mod routes;

use std::sync::Arc;

use aws_config::Region;
use aws_sdk_s3::config::Credentials;
use aws_sdk_s3::Client;
use axum::Router;
use backend::db::Db;
use backend::utils::log::init_tracing;
use backend::utils::router::RoutePacked;
use backend::State;
use backend::WithDb;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::trace::TraceLayer;
use tracing::info;

#[derive(Clone)]
struct AppState {
    db: Arc<Db>,
    storage_internal: Arc<Client>,
    storage_public: Arc<Client>,
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
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());

    let config = aws_sdk_s3::config::Builder::new()
        .behavior_version_latest()
        .region(Region::new("dz-center-1"))
        .endpoint_url("http://localhost:9000")
        .force_path_style(true)
        .credentials_provider(Credentials::new(
            dotenv!("STORAGE_USER"),
            dotenv!("STORAGE_PASSWORD"),
            None,
            None,
            "static",
        ));

    let storage_public = Arc::new(Client::from_conf(
        config
            .clone()
            .endpoint_url("https://cdn.benxo.test")
            .build(),
    ));

    let storage_internal = Arc::new(Client::from_conf(config.build()));

    let state = AppState {
        db: Arc::new(Db::new(dotenv!("DB_URI"), dotenv!("DB_NAME")).await),
        storage_internal,
        storage_public,
    };

    let app = Router::new()
        .nest_packed(routes::ApiRoutes::make_router())
        .layer(CookieManagerLayer::new())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    axum::serve(listener, app).await.unwrap();
}
