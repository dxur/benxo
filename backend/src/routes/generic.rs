use crate::db::*;
use crate::models::{Page, Pagination};
use axum::http::StatusCode;

use crate::AppState;

pub async fn create<M: CreatableInDb>(
    state: &AppState,
    body: impl Into<M::CreateInDb>,
) -> Result<M::Public, StatusCode> {
    let value = M::create(&state.db, body.into())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    M::on_create(&state, &value).await;
    Ok(value.into())
}

pub async fn get_all<M: ListableInDb>(
    state: &AppState,
    pagination: Pagination,
    body: impl Into<M::ListInDb>,
) -> Result<Page<M::Public>, StatusCode> {
    let data = M::get_all(
        &state.db,
        body.into(),
        pagination.limit(),
        pagination.offset(),
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Page {
        data: data.1.into_iter().map(|v| v.into()).collect(),
        total: data.0,
        per_page: pagination.per_page(),
        page: pagination.page(),
    })
}

pub async fn get_some<M: FilterableInDb>(
    state: &AppState,
    pagination: Pagination,
    body: impl Into<M::FilterInDb>,
) -> Result<Page<M::Public>, StatusCode> {
    let data = M::get_some(
        &state.db,
        body.into(),
        pagination.limit(),
        pagination.offset(),
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Page {
        data: data.1.into_iter().map(|v| v.into()).collect(),
        total: data.0,
        per_page: pagination.per_page(),
        page: pagination.page(),
    })
}

pub async fn get_one<M: FetchableInDb>(
    state: &AppState,
    body: impl Into<M::FetchInDb>,
) -> Result<M::Public, StatusCode> {
    Ok(M::get_one(&state.db, body.into())
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .ok_or(StatusCode::NOT_FOUND)?
        .into())
}

pub async fn update<M: UpdatableInDb>(
    state: &AppState,
    body: impl Into<M::UpdateInDb>,
) -> Result<M::Public, StatusCode> {
    let value = M::update(&state.db, body.into())
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .ok_or(StatusCode::NOT_FOUND)?;
    M::on_update(&state, &value.0, &value.1).await;
    Ok(value.1.into())
}

pub async fn delete<M: DeletableInDb>(
    state: &AppState,
    body: impl Into<M::DeleteInDb>,
) -> Result<M::Public, StatusCode> {
    let value = M::delete(&state.db, body.into())
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .ok_or(StatusCode::NOT_FOUND)?;
    M::on_delete(&state, &value.0, &value.1).await;
    Ok(value.1.into())
}
