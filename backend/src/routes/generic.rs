use crate::db::*;
use crate::models::{Page, Pagination};
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::Json;

use crate::AppState;

pub async fn create<M: CreatableInDb>(
    State(state): State<AppState>,
    Json(body): Json<impl Into<M::CreateInDb>>,
) -> Result<Json<M::Public>, StatusCode> {
    let value = M::create(&state.db, body.into())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    M::on_create(&state, &value).await;
    Ok(Json(value.into()))
}

pub async fn get_all<M: ListableInDb>(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
    Json(body): Json<impl Into<M::ListInDb>>,
) -> Result<Json<Page<M::Public>>, StatusCode> {
    let data = M::get_all(
        &state.db,
        body.into(),
        pagination.limit(),
        pagination.offset(),
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(Page {
        data: data.1.into_iter().map(|v| v.into()).collect(),
        total: data.0,
        per_page: pagination.per_page(),
        page: pagination.page(),
    }))
}

pub async fn get_some<M: FilterableInDb>(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
    Json(body): Json<impl Into<M::FilterInDb>>,
) -> Result<Json<Page<M::Public>>, StatusCode> {
    let data = M::get_some(
        &state.db,
        body.into(),
        pagination.limit(),
        pagination.offset(),
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(Page {
        data: data.1.into_iter().map(|v| v.into()).collect(),
        total: data.0,
        per_page: pagination.per_page(),
        page: pagination.page(),
    }))
}

pub async fn get_one<M: FetchableInDb>(
    State(state): State<AppState>,
    Json(body): Json<impl Into<M::FetchInDb>>,
) -> Result<Json<M::Public>, StatusCode> {
    Ok(Json(
        M::get_one(&state.db, body.into())
            .await
            .map_err(|_| StatusCode::BAD_REQUEST)?
            .ok_or(StatusCode::NOT_FOUND)?
            .into(),
    ))
}

pub async fn update<M: UpdatableInDb>(
    State(state): State<AppState>,
    Json(body): Json<impl Into<M::UpdateInDb>>,
) -> Result<Json<M::Public>, StatusCode> {
    let value = M::update(&state.db, body.into())
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .ok_or(StatusCode::NOT_FOUND)?;
    M::on_update(&state, &value.0, &value.1).await;
    Ok(Json(value.1.into()))
}

pub async fn delete<M: DeletableInDb>(
    State(state): State<AppState>,
    Json(body): Json<impl Into<M::DeleteInDb>>,
) -> Result<Json<M::Public>, StatusCode> {
    let value = M::delete(&state.db, body.into())
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .ok_or(StatusCode::NOT_FOUND)?;
    M::on_delete(&state, &value.0, &value.1).await;
    Ok(Json(value.1.into()))
}
