use axum::response::{IntoResponse, Response};
use axum::{extract::FromRequestParts, http::request::Parts};
use core::fmt;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use tower_cookies::Cookies;

use crate::utils::auth::{decode_access_token, decode_refresh_token};

#[derive(Debug)]
pub struct UnauthorizedRejection;

impl fmt::Display for UnauthorizedRejection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unautherized")
    }
}

impl IntoResponse for UnauthorizedRejection {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, self.to_string()).into_response()
    }
}

#[derive(Debug)]
pub struct StoreId(pub String);

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub store_id: String,
    pub email: String,
}

impl<S: Send + Sync> FromRequestParts<S> for StoreId {
    type Rejection = UnauthorizedRejection;

    async fn from_request_parts(_parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // TODO: extract the store_id
        return Ok(StoreId("some".to_string()));
    }
}

impl<S: Send + Sync> FromRequestParts<S> for UserData {
    type Rejection = UnauthorizedRejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let cookies = Cookies::from_request_parts(parts, state)
            .await
            .map_err(|_| UnauthorizedRejection)?;

        let access = cookies
            .get("access_token")
            .map(|c| c.value().to_string())
            .ok_or(UnauthorizedRejection)?;

        let access_token = decode_access_token(&access);
        if let Some(data) = access_token {
            return Ok(data);
        } else {
            let refresh = cookies
                .get("refresh_token")
                .map(|c| c.value().to_string())
                .ok_or(UnauthorizedRejection)?;
            let refresh_token = decode_refresh_token(&refresh);
            if let Some(_) = refresh_token {
                todo!("implement refresh token logic")
            }
        }

        Err(UnauthorizedRejection)
    }
}
