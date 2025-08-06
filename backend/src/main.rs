#![allow(dead_code)]
#![allow(unused)]
#![allow(async_fn_in_trait)]

// pub mod db;
pub mod events;
pub mod extractors;
// pub mod extractors;
pub mod middlewares;
// pub mod models;
pub mod platform;
// pub mod routes;
mod domain;
pub mod tenant;
pub mod types;
pub mod utils;
pub mod validators;

use axum::Router;
use bson::doc;
use mongodb::options::ClientOptions;
use mongodb::Client;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::trace::TraceLayer;
use tracing::{debug, info};

use crate::platform::business::routes::BusinessRoutes;
use crate::platform::business::service::BusinessService;
use crate::platform::user::repo::MongoUserRepo;
use crate::platform::user::routes::UserRoutes;
use crate::platform::user::service::UserService;
use crate::tenant::order::repo::MongoOrderRepo;
use crate::tenant::order::routes::OrderRoutes;
use crate::tenant::product::repo::MongoProductRepo;
use crate::tenant::product::routes::ProductRoutes;
use crate::tenant::product::service::ProductService;
use crate::tenant::store::repo::{MongoStoreRegRepo, MongoStoreRepo};
use crate::tenant::store::routes::{PubStoreRoutes, StoreRoutes};
use crate::tenant::store::service::StoreService;
use crate::utils::log::init_tracing;
use crate::utils::router::RoutePacked;
use crate::{platform::business::repo::MongoBusinessRepo, tenant::order::service::OrderService};

type AppState = Arc<State>;

struct State {
    pub user_service: UserService<MongoUserRepo>,
    pub business_service: BusinessService<MongoBusinessRepo>,
    pub product_service: ProductService<MongoProductRepo>,
    pub order_service: OrderService<MongoOrderRepo>,
    pub store_service: StoreService<MongoStoreRepo, MongoStoreRegRepo>,
    pub store_suffix: String,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    init_tracing();
    let db_uri = std::env::var("ATLAS_URI").unwrap();
    let root_db = std::env::var("ROOT_DB_NAME").unwrap();
    let store_suffix = format!(".{}", std::env::var("STORE_SUFFIX").unwrap());

    let api_listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("api listening on {}", api_listener.local_addr().unwrap());
    let internal_listener = TcpListener::bind("0.0.0.0:2000").await.unwrap();
    info!(
        "internal listening on {}",
        internal_listener.local_addr().unwrap()
    );
    let www_listener = TcpListener::bind("0.0.0.0:5080").await.unwrap();
    info!("www listening on {}", www_listener.local_addr().unwrap());

    let client_options = ClientOptions::parse(db_uri).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database(&root_db);

    // TODO: dev only
    {
        let admin_db = client.database("admin");
        let res = admin_db
            .run_command(doc! {
                "replSetInitiate": {}
            })
            .await;
        debug!("replset initialization result: {:?}", res);
    }

    let user_repo = MongoUserRepo::new(&db);
    let business_repo = MongoBusinessRepo::new(&db);
    let store_reg_repo = MongoStoreRegRepo::new(&db);
    let product_repo = MongoProductRepo::new(client.clone());
    let order_repo = MongoOrderRepo::new(client.clone());
    let store_repo = MongoStoreRepo::new(client);

    let user_service = UserService::new(user_repo);
    let business_service = BusinessService::new(business_repo);
    let product_service = ProductService::new(product_repo);
    let order_service = OrderService::new(order_repo);
    let store_service = StoreService::new(store_repo, store_reg_repo);
    let state = Arc::new(State {
        user_service,
        business_service,
        product_service,
        order_service,
        store_service,
        store_suffix,
    });

    let api = Router::new()
        .nest_packed(UserRoutes::make_router())
        .nest_packed(BusinessRoutes::make_router())
        .nest_packed(ProductRoutes::make_router())
        .nest_packed(OrderRoutes::make_router())
        .nest_packed(StoreRoutes::make_router())
        .layer(CookieManagerLayer::new())
        .layer(TraceLayer::new_for_http())
        .with_state(state.clone());

    let internal = Router::new()
        .route("/api/v1/tls", axum::routing::get(domain::ask_tls))
        .layer(TraceLayer::new_for_http())
        .with_state(state.clone());

    let www = Router::new()
        .merge(PubStoreRoutes::make_router().1)
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    tokio::select!(
        r = axum::serve(www_listener, www) => {r}
        r = axum::serve(internal_listener, internal) => {r}
        r = axum::serve(api_listener, api) => {r}
    )
    .unwrap();
}
