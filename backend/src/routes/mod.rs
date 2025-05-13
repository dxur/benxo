pub mod auth;
pub mod generic;

use axum::extract::Query;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;

use crate::api::*;
use crate::extractors::StoreId;
use crate::models::order::*;
use crate::models::product::*;
use crate::models::user::*;
use crate::models::*;
use crate::utils::types::IntoContext;
use crate::utils::types::WithContext;
use crate::AppState;

impl Routes<AppState> for ApiRoutes {
    // ---- Products ----
    async fn get_all_products(
        state: State<AppState>,
        StoreId(store): StoreId,
        pagination: Query<Pagination>,
    ) -> Result<Json<Page<ProductPublic>>, StatusCode> {
        generic::get_all::<Product>(state, pagination, Json(store.into_context())).await
    }

    async fn get_one_product(
        state: State<AppState>,
        StoreId(store): StoreId,
        Json(body): Json<ProductFetch>,
    ) -> Result<Json<ProductPublic>, StatusCode> {
        generic::get_one::<Product>(state, Json(body.with_context(store))).await
    }

    async fn create_product(
        state: State<AppState>,
        StoreId(store): StoreId,
        Json(body): Json<ProductCreate>,
    ) -> Result<Json<ProductPublic>, StatusCode> {
        generic::create::<Product>(state, Json(body.with_context(store))).await
    }

    async fn update_product(
        state: State<AppState>,
        StoreId(store): StoreId,
        Json(body): Json<ProductUpdate>,
    ) -> Result<Json<ProductPublic>, StatusCode> {
        generic::update::<Product>(state, Json(body.with_context(store))).await
    }

    async fn delete_product(
        state: State<AppState>,
        StoreId(store): StoreId,
        Json(body): Json<ProductDelete>,
    ) -> Result<Json<ProductPublic>, StatusCode> {
        generic::delete::<Product>(state, Json(body.with_context(store))).await
    }

    // ---- Users ----
    async fn get_all_users(
        state: State<AppState>,
        StoreId(store): StoreId,
        pagination: Query<Pagination>,
    ) -> Result<Json<Page<UserPublic>>, StatusCode> {
        // generic::get_all::<User>(state, pagination).await
        todo!()
    }

    async fn get_one_user(
        state: State<AppState>,
        StoreId(store): StoreId,
        body: Json<UserFetch>,
    ) -> Result<Json<UserPublic>, StatusCode> {
        generic::get_one::<User>(state, body).await
    }

    async fn create_user(
        state: State<AppState>,
        StoreId(store): StoreId,
        body: Json<UserCreate>,
    ) -> Result<Json<UserPublic>, StatusCode> {
        generic::create::<User>(state, body).await
    }

    async fn update_user(
        state: State<AppState>,
        StoreId(store): StoreId,
        body: Json<UserUpdate>,
    ) -> Result<Json<UserPublic>, StatusCode> {
        generic::update::<User>(state, body).await
    }

    async fn delete_user(
        state: State<AppState>,
        StoreId(store): StoreId,
        body: Json<UserDelete>,
    ) -> Result<Json<UserPublic>, StatusCode> {
        generic::delete::<User>(state, body).await
    }

    // ---- Orders ----
    async fn get_all_orders(
        state: State<AppState>,
        StoreId(store): StoreId,
        pagination: Query<Pagination>,
    ) -> Result<Json<Page<OrderPublic>>, StatusCode> {
        // generic::get_all::<Order>(state, pagination).await
        todo!()
    }

    async fn get_one_order(
        state: State<AppState>,
        StoreId(store): StoreId,
        body: Json<OrderFetch>,
    ) -> Result<Json<OrderPublic>, StatusCode> {
        generic::get_one::<Order>(state, body).await
    }

    async fn create_order(
        state: State<AppState>,
        StoreId(store): StoreId,
        body: Json<OrderCreate>,
    ) -> Result<Json<OrderPublic>, StatusCode> {
        generic::create::<Order>(state, body).await
    }

    async fn update_order(
        state: State<AppState>,
        StoreId(store): StoreId,
        body: Json<OrderUpdate>,
    ) -> Result<Json<OrderPublic>, StatusCode> {
        generic::update::<Order>(state, body).await
    }

    async fn delete_order(
        state: State<AppState>,
        StoreId(store): StoreId,
        body: Json<OrderDelete>,
    ) -> Result<Json<OrderPublic>, StatusCode> {
        generic::delete::<Order>(state, body).await
    }
}
