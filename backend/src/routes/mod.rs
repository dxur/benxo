pub mod auth;
pub mod generic;

use axum::extract::Query;
use axum::extract::State;
use axum::Json;
use macros::{route, routes};

use crate::extractors::StoreId;
use crate::models::order::*;
use crate::models::product::*;
use crate::models::settings::Settings;
use crate::models::settings::SettingsPublic;
use crate::models::*;
use crate::utils::types::IntoContext;
use crate::utils::types::WithContext;
use crate::AppState;

pub struct ApiRoutes;

#[routes(prefix = "/api", state = "AppState")]
impl ApiRoutes {
    // ---- Products ----
    #[route(method=get, path = "/products")]
    async fn get_all_products(
        state: State<AppState>,
        StoreId(store): StoreId,
        #[query] pagination: Pagination,
    ) -> Page<ProductPublic> {
        generic::get_all::<Product>(state, Query(pagination), Json(store.into_context())).await
    }

    #[route(method=post, path = "/products/")]
    async fn get_one_product(
        state: State<AppState>,
        StoreId(store): StoreId,
        #[json] body: ProductFetch,
    ) -> ProductPublic {
        generic::get_one::<Product>(state, Json(body.with_context(store))).await
    }

    #[route(method=post, path = "/products")]
    async fn create_product(
        state: State<AppState>,
        StoreId(store): StoreId,
        #[json] body: ProductCreate,
    ) -> ProductPublic {
        generic::create::<Product>(state, Json(body.with_context(store))).await
    }

    #[route(method=patch, path = "/products/")]
    async fn update_product(
        state: State<AppState>,
        StoreId(store): StoreId,
        #[json] body: ProductUpdate,
    ) -> ProductPublic {
        generic::update::<Product>(state, Json(body.with_context(store))).await
    }

    #[route(method=delete, path = "/products/")]
    async fn delete_product(
        state: State<AppState>,
        StoreId(store): StoreId,
        #[json] body: ProductDelete,
    ) -> ProductPublic {
        generic::delete::<Product>(state, Json(body.with_context(store))).await
    }

    // ---- Orders ----
    #[route(method=get, path = "/orders")]
    async fn get_all_orders(
        state: State<AppState>,
        StoreId(store): StoreId,
        #[query] pagination: Pagination,
    ) -> Page<OrderPublic> {
        generic::get_all::<Order>(state, Query(pagination), Json(store.into_context())).await
    }

    #[route(method=post, path = "/orders/")]
    async fn get_one_order(
        state: State<AppState>,
        StoreId(store): StoreId,
        #[json] body: OrderFetch,
    ) -> OrderPublic {
        generic::get_one::<Order>(state, Json(body.with_context(store))).await
    }

    #[route(method=post, path = "/orders")]
    async fn create_order(
        state: State<AppState>,
        StoreId(store): StoreId,
        #[json] body: OrderCreate,
    ) -> OrderPublic {
        generic::create::<Order>(state, Json(body.with_context(store))).await
    }

    #[route(method=patch, path = "/orders/")]
    async fn update_order(
        state: State<AppState>,
        StoreId(store): StoreId,
        #[json] body: OrderUpdate,
    ) -> OrderPublic {
        generic::update::<Order>(state, Json(body.with_context(store))).await
    }

    #[route(method=delete, path = "/orders/")]
    async fn delete_order(
        state: State<AppState>,
        StoreId(store): StoreId,
        #[json] body: OrderDelete,
    ) -> OrderPublic {
        generic::delete::<Order>(state, Json(body.with_context(store))).await
    }

    // ---- Settings ----
    #[route(method=get, path = "/settings")]
    async fn get_settings(state: State<AppState>, StoreId(store): StoreId) -> SettingsPublic {
        generic::get_one::<Settings>(state, Json(store.into_context())).await
    }
}
