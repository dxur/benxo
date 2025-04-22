use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::Json;
use common::models::{Page, Pagination};

use crate::models::{Model, ModelFilter};
use crate::AppState;

pub async fn create<M: Model>(
    State(state): State<AppState>,
    Json(body): Json<M::ModelCreate>,
) -> Result<Json<M::ModelPublic>, StatusCode> {
    let value = M::create_in_db(&state.db, body)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    M::on_create(&state, &value).await;
    Ok(Json(M::publish(value)))
}

pub async fn get_all<M: Model>(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Page<M::ModelPublic>>, StatusCode> {
    Ok(Json(Page {
        data: M::get_all_in_db(&state.db, pagination.limit(), pagination.offset())
            .await
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
            .into_iter()
            .map(|j| M::publish(j))
            .collect(),
        total: 0,
        per_page: pagination.limit(),
        page: pagination.offset(),
    }))
}

pub async fn get_some<M: ModelFilter>(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
    Json(body): Json<M::ModelFilter>,
) -> Result<Json<Vec<M::ModelPublic>>, StatusCode> {
    Ok(Json(
        M::get_some_in_db(&state.db, body, pagination.limit(), pagination.offset())
            .await
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
            .into_iter()
            .map(|j| M::publish(j))
            .collect(),
    ))
}

pub async fn get_one<M: Model>(
    State(state): State<AppState>,
    Json(body): Json<M::ModelFetch>,
) -> Result<Json<M::ModelPublic>, StatusCode> {
    Ok(Json(M::publish(
        M::get_one_in_db(&state.db, body)
            .await
            .map_err(|_| StatusCode::BAD_REQUEST)?
            .ok_or(StatusCode::NOT_FOUND)?,
    )))
}

pub async fn update<M: Model>(
    State(state): State<AppState>,
    Json(body): Json<M::ModelUpdate>,
) -> Result<Json<M::ModelPublic>, StatusCode> {
    let value = M::update_in_db(&state.db, &body)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .ok_or(StatusCode::NOT_FOUND)?;
    M::on_update(&state, &body, &value).await;
    Ok(Json(M::publish(value)))
}

pub async fn delete<M: Model>(
    State(state): State<AppState>,
    Json(body): Json<M::ModelDelete>,
) -> Result<Json<M::ModelPublic>, StatusCode> {
    let value = M::delete_in_db(&state.db, &body)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .ok_or(StatusCode::NOT_FOUND)?;
    M::on_delete(&state, body, &value).await;
    Ok(Json(M::publish(value)))
}
