#![allow(dead_code)]
#![allow(async_fn_in_trait)]

// pub mod db;
pub mod events;
pub mod extractors;
// pub mod extractors;
pub mod middlewares;
// pub mod models;
pub mod platform;
// pub mod routes;
pub mod types;
pub mod utils;
pub mod validators;

use std::sync::Arc;

use crate::platform::business::repo::MongoBusinessRepo;
use crate::platform::business::routes::BusinessRoutes;
use crate::platform::business::service::BusinessService;
use crate::platform::user::repo::MongoUserRepo;
use crate::platform::user::routes::UserRoutes;
use crate::platform::user::service::UserService;
use crate::utils::log::init_tracing;
use crate::utils::router::RoutePacked;
use axum::Router;
use mongodb::options::ClientOptions;
use mongodb::Client;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::trace::TraceLayer;
use tracing::info;

#[derive(Clone)]
struct AppState {
    pub user_service: Arc<UserService<MongoUserRepo>>,
    pub business_service: Arc<BusinessService<MongoBusinessRepo>>,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    init_tracing();
    let db_uri = std::env::var("ATLAS_URI").unwrap();
    let root_db = std::env::var("ROOT_DB_NAME").unwrap();
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());

    let client_options = ClientOptions::parse(db_uri).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database(&root_db);

    let user_repo = MongoUserRepo::new(&db);
    let business_repo = MongoBusinessRepo::new(&db);

    let user_service = Arc::new(UserService::new(user_repo));
    let business_service = Arc::new(BusinessService::new(business_repo));
    let state = AppState {
        user_service,
        business_service,
    };

    let app = Router::new()
        .nest_packed(UserRoutes::make_router())
        .nest_packed(BusinessRoutes::make_router())
        .layer(CookieManagerLayer::new())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    axum::serve(listener, app).await.unwrap();
}
