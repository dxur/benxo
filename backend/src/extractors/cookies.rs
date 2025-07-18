use std::marker::PhantomData;

use axum::{extract::FromRequestParts, http::request::Parts};
use tower_cookies::Cookies;

use crate::utils::error::ApiError;

pub struct FromCookies<T, F = T>(pub T, pub PhantomData<F>);

impl<T, F, S> FromRequestParts<S> for FromCookies<T, F>
where
    S: Send + Sync,
    F: for<'a> TryFrom<&'a Cookies>,
    for<'a> <F as TryFrom<&'a Cookies>>::Error: Into<ApiError>,
    T: TryFrom<F>,
    T::Error: Into<ApiError>,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let cookies = Cookies::from_request_parts(parts, state)
            .await
            .map_err(|_| ApiError::unauthorized(Option::<&str>::None))?;

        let f = F::try_from(&cookies).map_err(Into::into)?;
        let t = T::try_from(f).map_err(Into::into)?;
        Ok(Self(t, Default::default()))
    }
}
