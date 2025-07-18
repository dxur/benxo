#![allow(dead_code)]

#[macro_use]
extern crate dotenv_codegen;

mod graphql;

use std::sync::Arc;

use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::GraphQL;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use backend::db::Db;
use backend::utils::log::init_tracing;
use backend::State;
use backend::WithDb;
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

async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

#[tokio::main]
async fn main() {
    init_tracing();
    let listener = TcpListener::bind("0.0.0.0:3003").await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());

    // let state = AppState {
    //     db: Arc::new(Db::new(dotenv!("DB_URI"), dotenv!("DB_NAME")).await),
    // };

    // let schema = Schema::build(graphql::Query, EmptyMutation, EmptySubscription).finish();

    let app = Router::new()
        // .route("/graphql", get(graphiql).post_service(GraphQL::new(schema)))
        .layer(CookieManagerLayer::new())
        .layer(TraceLayer::new_for_http())
        // .with_state(state)
        ;

    axum::serve(listener, app).await.unwrap();
}
