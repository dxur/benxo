use axum::extract::{Path, Query, State};
use axum::response::{Html, IntoResponse};
use chrono::Utc;
use hyper::{header, HeaderMap};
use macros::routes;

use crate::extractors::cookies::FromCookies;
use crate::extractors::json::Json;
use crate::platform::business::api::BusinessSession;
use crate::types::id::Id;
use crate::utils::error::ApiResult;
use crate::AppState;
use tracing::trace;

use super::api::*;

pub struct StoreRoutes;

#[routes(prefix = "/api/v1/stores", state = AppState)]
impl StoreRoutes {}

pub struct PubStoreRoutes;

#[routes(prefix = "/", state = AppState)]
impl PubStoreRoutes {
    #[route(method=get, path="/")]
    async fn home() -> impl IntoResponse {
        Html("<h1>Comming soon</h1>")
    }
}
