use axum::http::StatusCode;

use crate::models::*;
use crate::models::{Page, Pagination};
use crate::WithDb;

pub async fn create<M: CreatableInDb>(
    state: &impl WithDb,
    body: impl Into<M::CreateInDb>,
) -> Result<M::InDb, StatusCode> {
    let value = M::create(&state.db(), None, body.into())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(value)
}

pub async fn get_all<M: ListableInDb>(
    state: &impl WithDb,
    pagination: Pagination,
    body: impl Into<M::ListInDb>,
) -> Result<Page<M::InDb>, StatusCode> {
    let data = M::get_all(
        &state.db(),
        None,
        body.into(),
        pagination.limit(),
        pagination.offset(),
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Page {
        data: data.1.into_iter().map(Into::into).collect(),
        total: Some(data.0),
        per_page: Some(pagination.per_page()),
        page: Some(pagination.page()),
        next_token: None,
    })
}

pub async fn get_some<M: FilterableInDb>(
    state: &impl WithDb,
    pagination: Pagination,
    body: impl Into<M::FilterInDb>,
) -> Result<Page<M::InDb>, StatusCode> {
    let data = M::get_some(
        &state.db(),
        None,
        body.into(),
        pagination.limit(),
        pagination.offset(),
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Page {
        data: data.1.into_iter().map(Into::into).collect(),
        total: Some(data.0),
        per_page: Some(pagination.per_page()),
        page: Some(pagination.page()),
        next_token: None,
    })
}

pub async fn get_one<M: FetchableInDb>(
    state: &impl WithDb,
    body: impl Into<M::FetchInDb>,
) -> Result<M::InDb, StatusCode> {
    Ok(M::get_one(&state.db(), None, body.into())
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .ok_or(StatusCode::NOT_FOUND)?)
}

pub async fn update<M: UpdatableInDb>(
    state: &impl WithDb,
    body: impl Into<M::UpdateInDb>,
) -> Result<M::InDb, StatusCode> {
    let value = M::update(&state.db(), None, body.into())
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(value.1)
}

pub async fn delete<M: DeletableInDb>(
    state: &impl WithDb,
    body: impl Into<M::DeleteInDb>,
) -> Result<M::InDb, StatusCode> {
    let value = M::delete(&state.db(), None, body.into())
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .ok_or(StatusCode::NOT_FOUND)?;
    M::on_delete(state, &value.0, &value.1).await;
    Ok(value.1)
}
