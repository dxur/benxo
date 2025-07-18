use axum::extract::State;
use axum::{extract::Path, extract::Query};
use macros::routes;

use crate::extractors::json::Json;
use crate::platform::business::api::*;
use crate::utils::error::ApiResult;
use crate::AppState;

pub struct BusinessRoutes;

#[routes(prefix = "/api/v1/businesses", state = AppState)]
impl BusinessRoutes {
    #[route(method = post, path = "/")]
    async fn create(State(state): State<AppState>) -> ApiResult<Json<BusinessView>> {
        todo!()
    }
}
