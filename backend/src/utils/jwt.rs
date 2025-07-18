use std::fmt::Debug;

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tracing::{error, trace};

use crate::utils::error::{ApiError, ApiResult};

const SECRET: &'static str = "secret";

#[derive(Debug, Serialize, Deserialize)]
struct Claims<T> {
    pub exp: usize,
    pub data: T,
}

pub fn encode_jwt<T: Serialize>(payload: T, ttl: Duration) -> ApiResult<String> {
    let exp = (Utc::now() + ttl).timestamp() as usize;
    let claims = Claims { exp, data: payload };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET.as_ref()),
    )
    .map_err(|_| ApiError::InternalError {
        message: "Can't encode the token".to_string(),
    })
}

pub fn decode_jwt<T: DeserializeOwned + Debug>(token: &str) -> ApiResult<T> {
    trace!("decode jwt: {:?}", token);
    decode::<Claims<T>>(
        token,
        &DecodingKey::from_secret(SECRET.as_ref()),
        &Validation::default(),
    )
    .map(|data: TokenData<Claims<T>>| data.claims.data)
    .map_err(|e| {
        error!("jwt decode error: {:?}", e);
        ApiError::InvalidRequestBody {
            expected: "A valid token".to_string(),
            message: "The Token you have Provided is invalid".to_string(),
        }
    })
    .map(|v| {
        trace!("decoded jwt: {:?}", v);
        v
    })
}
