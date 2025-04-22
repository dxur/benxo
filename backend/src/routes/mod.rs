pub mod auth;
pub mod generic;

use axum::extract::Query;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use common::models::product::*;
use common::models::{Page, Pagination};
use common::routes::*;

use crate::models::product::ProductModel;
use crate::AppState;

impl Routes<AppState> for ApiRoutes {
    async fn get_all_products(
        state: State<AppState>,
        pagination: Query<Pagination>,
    ) -> Result<Json<Page<ProductModelPublic>>, StatusCode> {
        generic::get_all::<ProductModel>(state, pagination).await
    }

    async fn get_one_product(
        state: State<AppState>,
        body: Json<ProductModelFetch>,
    ) -> Result<Json<ProductModelPublic>, StatusCode> {
        generic::get_one::<ProductModel>(state, body).await
    }

    async fn create_product(
        state: State<AppState>,
        body: Json<ProductModelCreate>,
    ) -> Result<Json<ProductModelPublic>, StatusCode> {
        generic::create::<ProductModel>(state, body).await
    }

    async fn update_product(
        state: State<AppState>,
        body: Json<ProductModelUpdate>,
    ) -> Result<Json<ProductModelPublic>, StatusCode> {
        generic::update::<ProductModel>(state, body).await
    }

    async fn delete_product(
        state: State<AppState>,
        body: Json<ProductModelDelete>,
    ) -> Result<Json<ProductModelPublic>, StatusCode> {
        generic::delete::<ProductModel>(state, body).await
    }
}
