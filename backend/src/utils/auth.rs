use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::extractors::UserData;

#[derive(Debug, Serialize, Deserialize)]
struct AccessClaims {
    sub: String,
    exp: usize,
}

impl AccessClaims {
    fn new(data: &UserData) -> Self {
        Self {
            sub: serde_json::to_string(data).unwrap(),
            exp: (chrono::Utc::now() + chrono::Duration::minutes(15)).timestamp() as usize,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct RefreshClaims {
    sub: String,
    exp: usize,
}

impl RefreshClaims {
    fn new(data: &UserData) -> Self {
        Self {
            sub: serde_json::to_string(data).unwrap(),
            exp: (chrono::Utc::now() + chrono::Duration::days(7)).timestamp() as usize,
        }
    }
}

pub fn issue_access_tokens(data: &UserData) -> Option<String> {
    let access_claims = AccessClaims::new(data);
    encode(
        &Header::default(),
        &access_claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )
    .ok()
}

pub fn issue_refresh_tokens(data: &UserData) -> Option<String> {
    let refresh_claims = RefreshClaims::new(data);
    encode(
        &Header::default(),
        &refresh_claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )
    .ok()
}

pub fn decode_access_token(token: &str) -> Option<UserData> {
    let token_data = decode::<AccessClaims>(
        token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    )
    .ok()?;
    serde_json::from_str(&token_data.claims.sub).ok()
}

pub fn decode_refresh_token(token: &str) -> Option<UserData> {
    let token_data = decode::<RefreshClaims>(
        token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    )
    .ok()?;
    serde_json::from_str(&token_data.claims.sub).ok()
}
