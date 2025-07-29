use axum::extract::{Path, Query, State};
use macros::routes;

use crate::extractors::cookies::FromCookies;
use crate::extractors::json::Json;
use crate::platform::business::api::BusinessSession;
use crate::platform::user::api::MessageResponse;
use crate::types::id::Id;
use crate::utils::error::ApiResult;
use crate::AppState;

use super::api::*;

pub struct ProductRoutes;

#[routes(prefix = "/api/v1/products", state = AppState)]
impl ProductRoutes {
    #[route(method=post, path="/create", res=ProductView)]
    async fn create_product(
        State(state): State<AppState>,
        FromCookies(business): FromCookies<BusinessSession>,
    ) -> ApiResult<Json<ProductView>> {
        state.product_service.create(business).await.map(Json)
    }

    #[route(method=post, path="/{id}", res=ProductView)]
    async fn get_product(
        State(state): State<AppState>,
        FromCookies(business): FromCookies<BusinessSession>,
        Path(product_id): Path<Id>,
    ) -> ApiResult<Json<ProductView>> {
        state
            .product_service
            .get_product(business, product_id)
            .await
            .map(Json)
    }

    #[route(method=post, path="/list", res=ProductListResponse)]
    async fn list_products(
        State(state): State<AppState>,
        FromCookies(business): FromCookies<BusinessSession>,
        Query(query): Query<ProductListQuery>,
    ) -> ApiResult<Json<ProductListResponse>> {
        state
            .product_service
            .list_products(business, query)
            .await
            .map(Json)
    }

    #[route(method=patch, path="/{id}", res=ProductView)]
    async fn edit_product(
        State(state): State<AppState>,
        FromCookies(business): FromCookies<BusinessSession>,
        Path(product_id): Path<Id>,
        #[json] update_req: ProductUpdate,
    ) -> ApiResult<Json<ProductView>> {
        state
            .product_service
            .update_product(business, product_id, update_req)
            .await
            .map(Json)
    }

    #[route(method=delete, path="/{id}", res=MessageResponse)]
    async fn delete_product(
        State(state): State<AppState>,
        FromCookies(business): FromCookies<BusinessSession>,
        Path(product_id): Path<Id>,
    ) -> ApiResult<Json<MessageResponse>> {
        state
            .product_service
            .delete_product(business, product_id)
            .await
            .map(|_| MessageResponse {
                message: "Product deleted successfully".to_string(),
            })
            .map(Json)
    }
}
