use axum::extract::State;
use axum::{extract::Path, extract::Query};
use macros::routes;

use crate::extractors::cookies::FromCookies;
use crate::extractors::json::Json;
use crate::platform::business::api::*;
use crate::platform::user::api::{MessageResponse, UserSession, UserToken};
use crate::utils::error::ApiResult;
use crate::AppState;

use super::api::{BusinessCreate, BusinessView, BusinessToken, BusinessSession};

pub struct BusinessRoutes;

#[routes(prefix = "/api/v1/businesses", state = AppState)]
impl BusinessRoutes {
    #[route(method = post, path = "/create", res = BusinessView)]
    async fn create(
        State(state): State<AppState>,
        FromCookies(token, _): FromCookies<UserSession, UserToken>,
        Json(create_req): Json<BusinessCreate>
    ) -> ApiResult<Json<BusinessView>> {
        state.business_service.create(token, create_req).await
    }

    #[route(method = post, path = "/current", res = BusinessView)]
    async fn current(
        State(state): State<AppState>,
        FromCookies(token, _): FromCookies<BusinessSession, BusinessToken>,
    ) -> ApiResult<Json<BusinessView>> {
        state.business_service.current(token).await
    }
}
