use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::Json;
use common::models::{Page, Pagination};

use crate::models::*;
use crate::AppState;

pub async fn create<M: Creatable>(
    State(state): State<AppState>,
    Json(body): Json<M::Create>,
) -> Result<Json<M::Public>, StatusCode> {
    let value = M::create(&state.db, body)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    M::on_create(&state, &value).await;
    Ok(Json(value.into()))
}

pub async fn get_all<M: Model>(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Page<M::Public>>, StatusCode> {
    let data = M::get_all(&state.db, pagination.limit(), pagination.offset())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(Page {
        data: data.1.into_iter().map(|v| v.into()).collect(),
        total: data.0,
        per_page: pagination.per_page(),
        page: pagination.page(),
    }))
}

pub async fn get_some<M: Filterable>(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
    Json(body): Json<M::Filter>,
) -> Result<Json<Page<M::Public>>, StatusCode> {
    let data = M::get_some(&state.db, body, pagination.limit(), pagination.offset())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(Page {
        data: data.1.into_iter().map(|v| v.into()).collect(),
        total: data.0,
        per_page: pagination.per_page(),
        page: pagination.page(),
    }))
}

pub async fn get_one<M: Fetchable>(
    State(state): State<AppState>,
    Json(body): Json<M::Fetch>,
) -> Result<Json<M::Public>, StatusCode> {
    Ok(Json(
        M::get_one(&state.db, body)
            .await
            .map_err(|_| StatusCode::BAD_REQUEST)?
            .ok_or(StatusCode::NOT_FOUND)?
            .into(),
    ))
}

pub async fn update<M: Updatable>(
    State(state): State<AppState>,
    Json(body): Json<M::Update>,
) -> Result<Json<M::Public>, StatusCode> {
    let value = M::update(&state.db, body)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .ok_or(StatusCode::NOT_FOUND)?;
    M::on_update(&state, &value.0, &value.1).await;
    Ok(Json(value.1.into()))
}

pub async fn delete<M: Deletable>(
    State(state): State<AppState>,
    Json(body): Json<M::Delete>,
) -> Result<Json<M::Public>, StatusCode> {
    let value = M::delete(&state.db, body)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .ok_or(StatusCode::NOT_FOUND)?;
    M::on_delete(&state, &value.0, &value.1).await;
    Ok(Json(value.1.into()))
}
