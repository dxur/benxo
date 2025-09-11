use axum::extract::State;
use axum::http::StatusCode;
use macros::routes;
use tower_cookies::Cookies;

use crate::extractors::cookies::FromCookies;
use crate::extractors::json::Json;
use crate::platform::user::api::*;
use crate::utils::error::ApiResult;
use crate::AppState;

pub struct UserRoutes;

#[routes(prefix = "/api/v1/users", state = AppState)]
impl UserRoutes {
    #[route(method = post, path = "/auth")]
    async fn auth(
        State(state): State<AppState>,
        cookies: Cookies,
        FromCookies(token): FromCookies<UserToken>,
        #[json] auth_req: AuthStep,
    ) -> ApiResult<Json<MessageResponse>> {
        let (token, msg) = state.user_service.auth(auth_req, token).await?;
        cookies.add(token.try_into()?);
        Ok(Json(msg))
    }

    #[route(method = delete, path = "/logout")]
    async fn logout(State(state): State<AppState>, cookies: Cookies) -> ApiResult<StatusCode> {
        cookies.add(UserToken::None.try_into()?);
        Ok(StatusCode::OK)
    }

    #[route(method = post, path = "/me", res = UserDto)]
    async fn me(
        State(state): State<AppState>,
        FromCookies(token): FromCookies<UserSession>,
    ) -> ApiResult<Json<UserDto>> {
        state.user_service.me(token.user_id).await.map(Json)
    }
}
