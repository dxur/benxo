use axum::extract::{Path, Query, State};
use macros::routes;

use crate::extractors::cookies::FromCookies;
use crate::extractors::json::Json;
use crate::platform::business::api::BusinessSession;
use crate::types::id::Id;
use crate::utils::error::ApiResult;
use crate::AppState;

use super::api::*;

pub struct OrderRoutes;

#[routes(prefix = "/api/v1/orders", state = AppState)]
impl OrderRoutes {
    #[route(method=post, path="/create", res=OrderView)]
    async fn create_order(
        State(state): State<AppState>,
        FromCookies(business): FromCookies<BusinessSession>,
        #[json] create_req: OrderCreate,
    ) -> ApiResult<Json<OrderView>> {
        state
            .order_service
            .create_order(&state.product_service, business, create_req)
            .await
            .map(Json)
    }

    #[route(method=get, path="/{id}", res=OrderView)]
    async fn get_order(
        State(state): State<AppState>,
        FromCookies(business): FromCookies<BusinessSession>,
        Path(order_id): Path<Id>,
    ) -> ApiResult<Json<OrderView>> {
        state
            .order_service
            .get_order(business, order_id)
            .await
            .map(Json)
    }

    #[route(method=get, path="/number/{order_number}", res=OrderView)]
    async fn get_order_by_number(
        State(state): State<AppState>,
        FromCookies(business): FromCookies<BusinessSession>,
        Path(order_id): Path<Id>,
    ) -> ApiResult<Json<OrderView>> {
        state
            .order_service
            .get_order(business, order_id)
            .await
            .map(Json)
    }

    #[route(method=get, path="/list", res=OrderListResponse)]
    async fn list_orders(
        State(state): State<AppState>,
        FromCookies(business): FromCookies<BusinessSession>,
        Query(query): Query<OrderListQuery>,
    ) -> ApiResult<Json<OrderListResponse>> {
        state
            .order_service
            .list_orders(business, query)
            .await
            .map(Json)
    }

    #[route(method=patch, path="/{id}", res=OrderView)]
    async fn update_order(
        State(state): State<AppState>,
        FromCookies(business): FromCookies<BusinessSession>,
        Path(order_id): Path<Id>,
        #[json] update_req: OrderUpdate,
    ) -> ApiResult<Json<OrderView>> {
        state
            .order_service
            .update_order(business, order_id, update_req)
            .await
            .map(Json)
    }

    #[route(method=patch, path="/{id}/status", res=OrderView)]
    async fn update_order_status(
        State(state): State<AppState>,
        FromCookies(business): FromCookies<BusinessSession>,
        Path(order_id): Path<Id>,
        #[json] status_update: OrderStatusUpdate,
    ) -> ApiResult<Json<OrderView>> {
        state
            .order_service
            .update_order_status(business, order_id, status_update)
            .await
            .map(Json)
    }

    #[route(method=post, path="/bulk/status", res=BulkUpdateResponse)]
    async fn bulk_update_order_status(
        State(state): State<AppState>,
        FromCookies(business): FromCookies<BusinessSession>,
        #[json] bulk_update: BulkOrderStatusUpdate,
    ) -> ApiResult<Json<BulkUpdateResponse>> {
        state
            .order_service
            .bulk_update_order_status(business, bulk_update)
            .await
            .map(Json)
    }

    #[route(method=get, path="/analytics", res=OrderAnalytics)]
    async fn get_analytics(
        State(state): State<AppState>,
        FromCookies(business): FromCookies<BusinessSession>,
        Query(query): Query<AnalyticsQuery>,
    ) -> ApiResult<Json<OrderAnalytics>> {
        state
            .order_service
            .get_analytics(business, query.date_from, query.date_to)
            .await
            .map(Json)
    }

    #[route(method=get, path="/customer/{email}", res=OrderListResponse)]
    async fn get_customer_orders(
        State(state): State<AppState>,
        FromCookies(business): FromCookies<BusinessSession>,
        Path(customer_email): Path<String>,
        Query(pagination): Query<PaginationQuery>,
    ) -> ApiResult<Json<OrderListResponse>> {
        state
            .order_service
            .get_customer_orders(business, customer_email, pagination.page, pagination.limit)
            .await
            .map(Json)
    }
}
