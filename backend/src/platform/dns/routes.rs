use axum::extract::State;
use macros::routes;

use crate::extractors::cookies::FromCookies;
use crate::extractors::json::Json;
use crate::platform::business::api::BusinessSession;
use crate::platform::user::api::UserSession;
use crate::utils::error::ApiResult;
use crate::AppState;

use super::api::*;

pub struct DnsRoutes;

#[routes(prefix = "/api/v1/dns", state = AppState)]
impl DnsRoutes {
    #[route(method = get, path = "/domains", res = DomainListResponse)]
    async fn get_business_domains(
        State(state): State<AppState>,
        FromCookies(business_session): FromCookies<BusinessSession>,
        FromCookies(user_session): FromCookies<UserSession>,
    ) -> ApiResult<Json<DomainListResponse>> {
        state
            .dns_service
            .get_business_domains(business_session, user_session)
            .await
            .map(Json)
    }
}
