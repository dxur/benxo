use crate::utils::error::ApiError;
use axum::{
    extract::{FromRequest, OptionalFromRequest, Request},
    response::IntoResponse,
};
use serde::{de::DeserializeOwned, Serialize};

pub struct Json<T>(pub T);

impl<T, S> FromRequest<S> for Json<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        <axum::Json<_> as FromRequest<S>>::from_request(req, state)
            .await
            .map(|axum::Json(v)| Json(v))
            .map_err(|_e| ApiError::validation("unknown", "Unknown validation error"))
    }
}

impl<T, S> OptionalFromRequest<S> for Json<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request(req: Request, state: &S) -> Result<Option<Self>, Self::Rejection> {
        <axum::Json<_> as OptionalFromRequest<S>>::from_request(req, state)
            .await
            .map(|v| v.map(|axum::Json(v)| Json(v)))
            .map_err(|_e| ApiError::validation("unknown", "Unknown validation error"))
    }
}

impl<T> IntoResponse for Json<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        axum::Json(self.0).into_response()
    }
}
