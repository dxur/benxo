use axum::{extract::FromRequestParts, http::request::Parts};
use tower_cookies::Cookies;

use crate::utils::error::ApiError;

pub struct FromCookies<T>(pub T);

impl<T, S> FromRequestParts<S> for FromCookies<T>
where
    S: Send + Sync,
    T: for<'a> TryFrom<&'a Cookies>,
    for<'a> <T as TryFrom<&'a Cookies>>::Error: Into<ApiError>,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let cookeies = Cookies::from_request_parts(parts, state)
            .await
            .map_err(|_| ApiError::missing_token())?;
        T::try_from(&cookeies).map(FromCookies).map_err(Into::into)
    }
}
