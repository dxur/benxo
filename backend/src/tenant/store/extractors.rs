use axum::http::header::HOST;
use axum::{extract::FromRequestParts, http::request::Parts};
use axum::{http::StatusCode, response::Html};

use super::api::*;
use crate::utils::error::ApiError;
use crate::utils::types::CowStr;
use crate::AppState;

pub struct Store(pub StoreRegDto);

impl FromRequestParts<AppState> for Store {
    type Rejection = (StatusCode, Html<CowStr>);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let host = parts
            .headers
            .get(HOST)
            .and_then(|v| v.to_str().ok())
            .ok_or(ApiError::invalid_header("Host", "Host header must be included").into())?;
        if let Some(slug) = host.strip_suffix(&state.store_suffix) {
            Ok(Store(
                state
                    .store_service
                    .get_slug(slug)
                    .await
                    .map_err(Into::into)?,
            ))
        } else {
            Ok(Store(
                state
                    .store_service
                    .get_domain(host)
                    .await
                    .map_err(Into::into)?,
            ))
        }
    }
}
