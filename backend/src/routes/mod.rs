pub mod auth;
pub mod generic;

use axum::extract::Query;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use common::models::product::*;
use common::models::user::*;
use common::models::{Page, Pagination};
use common::routes::*;

use crate::models::product::ProductModel;
use crate::AppState;

impl Routes<AppState> for ApiRoutes {
    // ---- Products ----
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

    // ---- Product Variants ----
    async fn get_some_variants(
        state: State<AppState>,
        pagination: Query<Pagination>,
        body: Json<ProductVarModelFilter>,
    ) -> Result<Json<Page<ProductVarModelPublic>>, StatusCode> {
        generic::get_some::<ProductVarModel>(state, pagination, body).await
    }

    async fn get_one_variant(
        state: State<AppState>,
        body: Json<ProductVarModelFetch>,
    ) -> Result<Json<ProductVarModelPublic>, StatusCode> {
        generic::get_one::<ProductVarModel>(state, body).await
    }

    async fn create_variant(
        state: State<AppState>,
        body: Json<ProductVarModelCreate>,
    ) -> Result<Json<ProductVarModelPublic>, StatusCode> {
        generic::create::<ProductVarModel>(state, body).await
    }

    async fn update_variant(
        state: State<AppState>,
        body: Json<ProductVarModelUpdate>,
    ) -> Result<Json<ProductVarModelPublic>, StatusCode> {
        generic::update::<ProductVarModel>(state, body).await
    }

    async fn delete_variant(
        state: State<AppState>,
        body: Json<ProductVarModelDelete>,
    ) -> Result<Json<ProductVarModelPublic>, StatusCode> {
        generic::delete::<ProductVarModel>(state, body).await
    }

    // ---- Users ----
    async fn get_all_users(
        state: State<AppState>,
        pagination: Query<Pagination>,
    ) -> Result<Json<Page<UserModelPublic>>, StatusCode> {
        generic::get_all::<UserModel>(state, pagination).await
    }

    async fn get_one_user(
        state: State<AppState>,
        body: Json<UserModelFetch>,
    ) -> Result<Json<UserModelPublic>, StatusCode> {
        generic::get_one::<UserModel>(state, body).await
    }

    async fn create_user(
        state: State<AppState>,
        body: Json<UserModelCreate>,
    ) -> Result<Json<UserModelPublic>, StatusCode> {
        generic::create::<UserModel>(state, body).await
    }

    async fn update_user(
        state: State<AppState>,
        body: Json<UserModelUpdate>,
    ) -> Result<Json<UserModelPublic>, StatusCode> {
        generic::update::<UserModel>(state, body).await
    }

    async fn delete_user(
        state: State<AppState>,
        body: Json<UserModelDelete>,
    ) -> Result<Json<UserModelPublic>, StatusCode> {
        generic::delete::<UserModel>(state, body).await
    }
}
