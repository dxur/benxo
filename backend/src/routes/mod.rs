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

use crate::models::product::*;
use crate::models::*;
use crate::AppState;

impl Routes<AppState> for ApiRoutes {
    // ---- Products ----
    async fn get_all_products(
        state: State<AppState>,
        pagination: Query<Pagination>,
    ) -> Result<Json<Page<ProductPublic>>, StatusCode> {
        generic::get_all::<Product>(state, pagination).await
    }

    async fn get_one_product(
        state: State<AppState>,
        body: Json<ProductFetch>,
    ) -> Result<Json<ProductPublic>, StatusCode> {
        generic::get_one::<Product>(state, body).await
    }

    async fn create_product(
        state: State<AppState>,
        body: Json<ProductCreate>,
    ) -> Result<Json<ProductPublic>, StatusCode> {
        generic::create::<Product>(state, body).await
    }

    async fn update_product(
        state: State<AppState>,
        body: Json<ProductUpdate>,
    ) -> Result<Json<ProductPublic>, StatusCode> {
        generic::update::<Product>(state, body).await
    }

    async fn delete_product(
        state: State<AppState>,
        body: Json<ProductDelete>,
    ) -> Result<Json<ProductPublic>, StatusCode> {
        generic::delete::<Product>(state, body).await
    }

    // ---- Product Variants ----
    async fn get_some_variants(
        state: State<AppState>,
        pagination: Query<Pagination>,
        body: Json<ProductVarFilter>,
    ) -> Result<Json<Page<ProductVarPublic>>, StatusCode> {
        generic::get_some::<ProductVar>(state, pagination, body).await
    }

    async fn get_one_variant(
        state: State<AppState>,
        body: Json<ProductVarFetch>,
    ) -> Result<Json<ProductVarPublic>, StatusCode> {
        generic::get_one::<ProductVar>(state, body).await
    }

    async fn create_variant(
        state: State<AppState>,
        body: Json<ProductVarCreate>,
    ) -> Result<Json<ProductVarPublic>, StatusCode> {
        generic::create::<ProductVar>(state, body).await
    }

    async fn update_variant(
        state: State<AppState>,
        body: Json<ProductVarUpdate>,
    ) -> Result<Json<ProductVarPublic>, StatusCode> {
        generic::update::<ProductVar>(state, body).await
    }

    async fn delete_variant(
        state: State<AppState>,
        body: Json<ProductVarDelete>,
    ) -> Result<Json<ProductVarPublic>, StatusCode> {
        generic::delete::<ProductVar>(state, body).await
    }

    // ---- Users ----
    async fn get_all_users(
        state: State<AppState>,
        pagination: Query<Pagination>,
    ) -> Result<Json<Page<UserPublic>>, StatusCode> {
        generic::get_all::<User>(state, pagination).await
    }

    async fn get_one_user(
        state: State<AppState>,
        body: Json<UserFetch>,
    ) -> Result<Json<UserPublic>, StatusCode> {
        generic::get_one::<User>(state, body).await
    }

    async fn create_user(
        state: State<AppState>,
        body: Json<UserCreate>,
    ) -> Result<Json<UserPublic>, StatusCode> {
        generic::create::<User>(state, body).await
    }

    async fn update_user(
        state: State<AppState>,
        body: Json<UserUpdate>,
    ) -> Result<Json<UserPublic>, StatusCode> {
        generic::update::<User>(state, body).await
    }

    async fn delete_user(
        state: State<AppState>,
        body: Json<UserDelete>,
    ) -> Result<Json<UserPublic>, StatusCode> {
        generic::delete::<User>(state, body).await
    }
}
