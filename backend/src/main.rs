#![allow(dead_code)]
#![allow(unused)]
#![allow(async_fn_in_trait)]

// mod db;
mod events;
mod extractors;
// mod extractors;
mod middlewares;
// mod models;
mod platform;
// mod routes;
mod domain;
mod tenant;
mod types;
mod utils;
mod validators;

use std::sync::Arc;

use axum::Router;
use bson::doc;
use hickory_resolver::config::*;
use hickory_resolver::name_server::TokioConnectionProvider;
use hickory_resolver::Resolver;
use mongodb::options::ClientOptions as MongoClientOptions;
use mongodb::Client as MongoClient;
use s3::bucket::Bucket;
use s3::bucket_ops::BucketConfiguration;
use s3::creds::Credentials;
use s3::Region;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::trace::TraceLayer;
use tracing::{debug, info};

use crate::platform::business::routes::BusinessRoutes;
use crate::platform::business::service::BusinessService;
// use crate::platform::dns::repo::MongoDomainRepo;
// use crate::platform::dns::routes::DnsRoutes;
// use crate::platform::dns::service::DnsService;
use crate::platform::user::repo::MongoUserRepo;
use crate::platform::user::routes::UserRoutes;
use crate::platform::user::service::UserService;
use crate::tenant::file::repo::MongoFileRepo;
use crate::tenant::file::routes::FileRoutes;
use crate::tenant::file::service::FileService;
use crate::tenant::order::repo::MongoOrderRepo;
use crate::tenant::order::routes::{PubOrderRoutes, OrderRoutes};
use crate::tenant::product::repo::MongoProductRepo;
use crate::tenant::product::routes::{PubProductRoutes, ProductRoutes};
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
    // pub dns_service: DnsService<MongoDomainRepo>,
    pub product_service: ProductService<MongoProductRepo>,
    pub order_service: OrderService<MongoOrderRepo>,
    pub store_service: StoreService<MongoStoreRepo, MongoStoreRegRepo>,
    pub file_service: FileService<MongoFileRepo>,
    pub store_suffix: String,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    init_tracing();
    let db_uri = std::env::var("ATLAS_URI").unwrap();
    info!("ATLAS_URI = {}", db_uri);
    let root_db = std::env::var("ROOT_DB_NAME").unwrap();
    info!("ROOT_DB_NAME = {}", root_db);
    let store_suffix = format!(".{}", std::env::var("STORE_SUFFIX").unwrap());
    info!("STORE_SUFFIX = {}", store_suffix);
    let s3_uri = std::env::var("STORAGE_URI").unwrap();
    info!("STORAGE_URI = {}", s3_uri);
    let s3_region = std::env::var("STORAGE_REGION").unwrap();
    info!("STORAGE_REGION = {}", s3_region);
    let s3_user = std::env::var("STORAGE_USER").unwrap();
    info!("STORAGE_USER = {}", s3_user);
    let s3_bucket = std::env::var("STORAGE_BUCKET_NAME").unwrap();
    info!("STORAGE_BUCKET_NAME = {}", s3_bucket);
    let s3_pass = std::env::var("STORAGE_PASSWORD").unwrap();
    info!("STORAGE_PASSWORD = {}", s3_pass);

    let api_listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("api listening on {}", api_listener.local_addr().unwrap());
    let internal_listener = TcpListener::bind("0.0.0.0:2000").await.unwrap();
    info!(
        "internal listening on {}",
        internal_listener.local_addr().unwrap()
    );
    let www_listener = TcpListener::bind("0.0.0.0:5080").await.unwrap();
    info!("www listening on {}", www_listener.local_addr().unwrap());

    let mongo_client =
        MongoClient::with_options(MongoClientOptions::parse(db_uri).await.unwrap()).unwrap();
    let db = mongo_client.database(&root_db);

    let region = Region::Custom {
        endpoint: s3_uri,
        region: s3_region,
    };
    let credentials = Credentials::new(Some(&s3_user), Some(&s3_pass), None, None, None).unwrap();

    // TODO: dev only
    {
        let admin_db = mongo_client.database("admin");
        let res = admin_db
            .run_command(doc! {
                "replSetInitiate": {}
            })
            .await;
        debug!("replset initialization result: {:#?}", res);

        let res = Bucket::create_with_path_style(
            &s3_bucket,
            region.clone(),
            credentials.clone(),
            Default::default(),
        )
        .await;
        res.map(|v| {
            debug!("bucket creation result: {:#?}", v.bucket);
        })
        .map_err(|e| {
            debug!("bucket creation error: {:#?}", e);
        });
    }

    let bucket = *Bucket::new(&s3_bucket, region, credentials)
        .unwrap()
        .with_path_style();

    let resolver = Resolver::builder_with_config(
        hickory_resolver::system_conf::read_system_conf()
            .map(|(c, _)| c)
            .unwrap_or(ResolverConfig::default()),
        TokioConnectionProvider::default(),
    )
    .build();

    let user_repo = MongoUserRepo::new(&db);
    let business_repo = MongoBusinessRepo::new(&db);
    // let domain_repo = MongoDomainRepo::new(&db);
    let store_reg_repo = MongoStoreRegRepo::new(&db);
    let product_repo = MongoProductRepo::new(mongo_client.clone());
    let order_repo = MongoOrderRepo::new(mongo_client.clone());
    let store_repo = MongoStoreRepo::new(mongo_client.clone());
    let file_repo = MongoFileRepo::new(mongo_client);

    let user_service = UserService::new(user_repo);
    let business_service = BusinessService::new(business_repo);
    // let dns_service = DnsService::new(domain_repo, resolver);
    let product_service = ProductService::new(product_repo);
    let order_service = OrderService::new(order_repo);
    let store_service = StoreService::new(store_repo, store_reg_repo, resolver);
    let file_service = FileService::new(file_repo, bucket);

    let state = Arc::new(State {
        user_service,
        business_service,
        // dns_service,
        product_service,
        order_service,
        store_service,
        file_service,
        store_suffix,
    });

    let api = Router::new()
        .route("/api/v1/health", axum::routing::get(|| async { "OK" }))
        .nest_packed(UserRoutes::make_router())
        .nest_packed(BusinessRoutes::make_router())
        // .nest_packed(DnsRoutes::make_router())
        .nest_packed(ProductRoutes::make_router())
        .nest_packed(OrderRoutes::make_router())
        .nest_packed(StoreRoutes::make_router())
        .nest_packed(FileRoutes::make_router())
        .layer(CookieManagerLayer::new())
        .layer(TraceLayer::new_for_http())
        .with_state(state.clone());

    let internal = Router::new()
        .route("/api/v1/tls", axum::routing::get(domain::ask_tls))
        .layer(TraceLayer::new_for_http())
        .with_state(state.clone());

    let www = Router::new()
        .merge(PubStoreRoutes::make_router().1)
        .nest_packed(PubProductRoutes::make_router())
        .nest_packed(PubOrderRoutes::make_router())
        // .layer(CookieManagerLayer::new())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    tokio::select!(
        r = axum::serve(www_listener, www) => {r}
        r = axum::serve(internal_listener, internal) => {r}
        r = axum::serve(api_listener, api) => {r}
    )
    .unwrap();
}
