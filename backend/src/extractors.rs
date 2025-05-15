use axum::response::{IntoResponse, Response};
use axum::{extract::FromRequestParts, http::request::Parts};
use core::fmt;
use hyper::StatusCode;

pub struct StoreId(pub String);

#[derive(Debug)]
pub struct StoreIdRejection;

impl fmt::Display for StoreIdRejection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid or missing store ID in subdomain")
    }
}

impl IntoResponse for StoreIdRejection {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, self.to_string()).into_response()
    }
}

impl<S> FromRequestParts<S> for StoreId
where
    S: Send + Sync,
{
    type Rejection = StoreIdRejection;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let host = parts.headers.get("host").and_then(|v| v.to_str().ok());
        tracing::debug!("Extracted host header: {:?}", host);

        if let Some(host) = host {
            tracing::info!("Host: {}", host);
            let base_domain = dotenv!("APP_HOST");
            tracing::debug!("Using base domain: {}", base_domain);
            if let Some(subdomain) = host.strip_suffix(base_domain) {
                tracing::debug!("Matched subdomain: {}", subdomain);
                let sub = subdomain.trim_end_matches('.');

                if !sub.is_empty() {
                    // Uncomment to validate
                    // let is_valid = regex::Regex::new(r"^[a-z0-9]([-a-z0-9]{0,61}[a-z0-9])?$").unwrap();
                    // debug!("Validation result: {}", is_valid.is_match(subdomain));
                    // if is_valid.is_match(subdomain) {
                    return Ok(StoreId(sub.to_string()));
                    // }
                } else {
                    tracing::debug!("Subdomain is empty after stripping base domain.");
                }
            } else {
                tracing::debug!("Host does not end with base domain.");
            }
        } else {
            tracing::debug!("Host header missing or invalid.");
        }

        Err(StoreIdRejection)
    }
}
