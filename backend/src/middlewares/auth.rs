use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_extra::extract::CookieJar;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use tracing::info;

const SECRET: &[u8] = b"supersecretkey";

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub async fn auth_middleware(req: Request, next: Next) -> Response {
    let cookies = CookieJar::from_headers(req.headers());
    let token = cookies.get("token").map(|c| c.value().to_string());
    if let Some(token) = token {
        match decode::<Claims>(
            &token,
            &DecodingKey::from_secret(SECRET),
            &Validation::default(),
        ) {
            Ok(data) => {
                info!("Auth successful, {}", data.claims.sub);
                return next.run(req).await;
            }
            Err(_) => {}
        }
    }

    info!("Auth failed");
    (StatusCode::UNAUTHORIZED, "Unauthorized").into_response()
}
