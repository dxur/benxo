use axum::response::{IntoResponse, Response};
use axum::{extract::FromRequestParts, http::request::Parts};
use bson::oid::ObjectId;
use core::fmt;
use hyper::header::HOST;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use tower_cookies::Cookies;

use crate::db::store::StoreInDb;
use crate::db::FetchableInDb;
use crate::models::store::{Store, StoreFetch};
use crate::utils::auth::{decode_access_token, decode_refresh_token};
use crate::AppState;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub business_id: ObjectId,
    pub email: String,
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
            if let Some(r) = refresh_token {
                return Ok(r);
            }
        }

        Err(UnauthorizedRejection)
    }
}

#[derive(Debug)]
pub struct StoreId(pub String);

impl<S: Send + Sync> FromRequestParts<S> for StoreId {
    type Rejection = UnauthorizedRejection;

    async fn from_request_parts(_parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        return Ok(StoreId("some".to_string()));
    }
}

#[derive(Debug)]
pub struct NotFoundRejection;

impl fmt::Display for NotFoundRejection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "This content is no longger exist\n404 Not found")
    }
}

impl IntoResponse for NotFoundRejection {
    fn into_response(self) -> Response {
        (StatusCode::NOT_FOUND, self.to_string()).into_response()
    }
}

#[derive(Debug)]
pub struct StoreMeta(pub StoreInDb);

impl FromRequestParts<AppState> for StoreMeta {
    type Rejection = NotFoundRejection;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let host = parts
            .headers
            .get(HOST)
            .and_then(|h| h.to_str().ok())
            .ok_or(Self::Rejection {})?;

        let suffix = ".mystore.localhost";

        let store = if let Some(stripped) = host.strip_suffix(suffix) {
            StoreFetch::Id(stripped.to_string())
        } else {
            StoreFetch::Domain(host.to_string())
        };

        Ok(StoreMeta(
            Store::get_one(&state.db, store.into())
                .await
                .map_err(|_| Self::Rejection {})?
                .ok_or(Self::Rejection {})?,
        ))
    }
}
