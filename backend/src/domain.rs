use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;

use crate::AppState;

#[derive(Deserialize)]
pub struct Domain {
    pub domain: String,
}

pub async fn ask_tls(
    State(state): State<AppState>,
    Query(Domain { domain }): Query<Domain>,
) -> impl IntoResponse {
    state
        .store_service
        .get_domain(&domain)
        .await
        .map(|_| StatusCode::OK)
        .map_err(Into::<StatusCode>::into)
}
