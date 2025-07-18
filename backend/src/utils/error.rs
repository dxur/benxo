use std::{collections::HashMap, convert::Infallible};

use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use tracing::debug;

/// RFC 7807 Problem Details for HTTP APIs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemDetails {
    /// A URI reference that identifies the problem type
    #[serde(rename = "type")]
    pub problem_type: String,

    /// A short, human-readable summary of the problem type
    pub title: String,

    /// The HTTP status code
    pub status: u16,

    /// A human-readable explanation specific to this occurrence
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,

    /// A URI reference that identifies the specific occurrence
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,

    /// Additional problem-specific extension fields
    #[serde(flatten)]
    pub extensions: HashMap<String, serde_json::Value>,
}

impl ProblemDetails {
    pub fn new(problem_type: impl Into<String>, title: impl Into<String>, status: u16) -> Self {
        Self {
            problem_type: problem_type.into(),
            title: title.into(),
            status,
            detail: None,
            instance: None,
            extensions: HashMap::new(),
        }
    }

    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }

    pub fn with_instance(mut self, instance: impl Into<String>) -> Self {
        self.instance = Some(instance.into());
        self
    }

    pub fn with_extension(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }

    /// Content-Type header value for RFC 7807
    pub const CONTENT_TYPE: &'static str = "application/problem+json";
}

/// Application-specific error types
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Validation failed")]
    ValidationError { field: String, message: String },

    #[error("Resource not found")]
    NotFound { resource: String, id: String },

    #[error("Authentication failed")]
    Unauthorized { reason: String },

    #[error("Access denied")]
    Forbidden { resource: String, action: String },

    #[error("Conflict occurred")]
    Conflict { resource: String, reason: String },

    #[error("Rate limit exceeded")]
    RateLimitExceeded { retry_after: u64 },

    #[error("Internal server error")]
    InternalError { message: String },

    #[error("Service unavailable")]
    ServiceUnavailable {
        service: String,
        retry_after: Option<u64>,
    },

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("External service error: {0}")]
    ExternalServiceError(String),

    // NEW: Syntax and parsing errors
    #[error("Invalid JSON syntax")]
    InvalidJson {
        message: String,
        line: Option<usize>,
        column: Option<usize>,
    },

    #[error("Invalid request body")]
    InvalidRequestBody { expected: String, message: String },

    #[error("Missing required field")]
    MissingField { field: String },

    #[error("Invalid content type")]
    InvalidContentType { expected: String, received: String },

    #[error("Request body too large")]
    RequestTooLarge {
        max_size: usize,
        received_size: usize,
    },

    #[error("Invalid query parameter")]
    InvalidQueryParam { param: String, message: String },

    #[error("Invalid path parameter")]
    InvalidPathParam { param: String, message: String },

    #[error("Invalid header")]
    InvalidHeader { header: String, message: String },

    #[error("Malformed request")]
    MalformedRequest { message: String },
}

impl ApiError {
    /// Convert ApiError to RFC 7807 ProblemDetails
    pub fn to_problem_details(&self) -> ProblemDetails {
        debug!("Returning error: {:?}", self);
        match self {
            ApiError::ValidationError { field, message } => ProblemDetails::new(
                "https://example.com/problems/validation-error",
                "Validation Error",
                400,
            )
            .with_detail(message)
            .with_extension("field", serde_json::Value::String(field.clone())),

            ApiError::NotFound { resource, id } => ProblemDetails::new(
                "https://example.com/problems/not-found",
                "Resource Not Found",
                404,
            )
            .with_detail(&format!("{} with id '{}' was not found", resource, id))
            .with_extension("resource", serde_json::Value::String(resource.clone()))
            .with_extension("id", serde_json::Value::String(id.clone())),

            ApiError::Unauthorized { reason } => ProblemDetails::new(
                "https://example.com/problems/unauthorized",
                "Authentication Required",
                401,
            )
            .with_detail(reason),

            ApiError::Forbidden { resource, action } => ProblemDetails::new(
                "https://example.com/problems/forbidden",
                "Access Denied",
                403,
            )
            .with_detail(&format!("Access denied for {} on {}", action, resource))
            .with_extension("resource", serde_json::Value::String(resource.clone()))
            .with_extension("action", serde_json::Value::String(action.clone())),

            ApiError::Conflict { resource, reason } => ProblemDetails::new(
                "https://example.com/problems/conflict",
                "Resource Conflict",
                409,
            )
            .with_detail(&format!("Conflict with {}: {}", resource, reason))
            .with_extension("resource", serde_json::Value::String(resource.clone())),

            ApiError::RateLimitExceeded { retry_after } => ProblemDetails::new(
                "https://example.com/problems/rate-limit-exceeded",
                "Rate Limit Exceeded",
                429,
            )
            .with_detail("Too many requests. Please try again later.")
            .with_extension(
                "retry_after",
                serde_json::Value::Number((*retry_after).into()),
            ),

            ApiError::InternalError { message: _ } => {
                ProblemDetails::new(
                    "https://example.com/problems/internal-error",
                    "Internal Server Error",
                    500,
                )
                .with_detail("An unexpected error occurred")
                // Don't expose internal error details in production
                .with_extension(
                    "error_id",
                    serde_json::Value::String(ObjectId::new().to_hex()),
                )
            }

            ApiError::ServiceUnavailable {
                service,
                retry_after,
            } => {
                let mut problem = ProblemDetails::new(
                    "https://example.com/problems/service-unavailable",
                    "Service Unavailable",
                    503,
                )
                .with_detail(&format!(
                    "The {} service is temporarily unavailable",
                    service
                ))
                .with_extension("service", serde_json::Value::String(service.clone()));

                if let Some(retry) = retry_after {
                    problem = problem
                        .with_extension("retry_after", serde_json::Value::Number((*retry).into()));
                }

                problem
            }

            ApiError::DatabaseError(_) => ProblemDetails::new(
                "https://example.com/problems/database-error",
                "Database Error",
                500,
            )
            .with_detail("A database error occurred")
            .with_extension(
                "error_id",
                serde_json::Value::String(ObjectId::new().to_hex()),
            ),

            ApiError::ExternalServiceError(_) => ProblemDetails::new(
                "https://example.com/problems/external-service-error",
                "External Service Error",
                502,
            )
            .with_detail("An external service error occurred")
            .with_extension(
                "error_id",
                serde_json::Value::String(ObjectId::new().to_hex()),
            ),

            // NEW: Handle syntax and parsing errors
            ApiError::InvalidJson {
                message,
                line,
                column,
            } => {
                let mut problem = ProblemDetails::new(
                    "https://example.com/problems/invalid-json",
                    "Invalid JSON Syntax",
                    400,
                )
                .with_detail(&format!("JSON parsing failed: {}", message));

                if let Some(line) = line {
                    problem =
                        problem.with_extension("line", serde_json::Value::Number((*line).into()));
                }
                if let Some(column) = column {
                    problem = problem
                        .with_extension("column", serde_json::Value::Number((*column).into()));
                }

                problem
            }

            ApiError::InvalidRequestBody { expected, message } => ProblemDetails::new(
                "https://example.com/problems/invalid-request-body",
                "Invalid Request Body",
                400,
            )
            .with_detail(&format!("Request body validation failed: {}", message))
            .with_extension(
                "expected_format",
                serde_json::Value::String(expected.clone()),
            ),

            ApiError::MissingField { field } => ProblemDetails::new(
                "https://example.com/problems/missing-field",
                "Missing Required Field",
                400,
            )
            .with_detail(&format!("Required field '{}' is missing", field))
            .with_extension("field", serde_json::Value::String(field.clone())),

            ApiError::InvalidContentType { expected, received } => ProblemDetails::new(
                "https://example.com/problems/invalid-content-type",
                "Invalid Content Type",
                415,
            )
            .with_detail(&format!(
                "Expected '{}' but received '{}'",
                expected, received
            ))
            .with_extension("expected", serde_json::Value::String(expected.clone()))
            .with_extension("received", serde_json::Value::String(received.clone())),

            ApiError::RequestTooLarge {
                max_size,
                received_size,
            } => ProblemDetails::new(
                "https://example.com/problems/request-too-large",
                "Request Entity Too Large",
                413,
            )
            .with_detail(&format!(
                "Request body size {} exceeds maximum allowed size {}",
                received_size, max_size
            ))
            .with_extension("max_size", serde_json::Value::Number((*max_size).into()))
            .with_extension(
                "received_size",
                serde_json::Value::Number((*received_size).into()),
            ),

            ApiError::InvalidQueryParam { param, message } => ProblemDetails::new(
                "https://example.com/problems/invalid-query-param",
                "Invalid Query Parameter",
                400,
            )
            .with_detail(&format!("Invalid query parameter '{}': {}", param, message))
            .with_extension("parameter", serde_json::Value::String(param.clone())),

            ApiError::InvalidPathParam { param, message } => ProblemDetails::new(
                "https://example.com/problems/invalid-path-param",
                "Invalid Path Parameter",
                400,
            )
            .with_detail(&format!("Invalid path parameter '{}': {}", param, message))
            .with_extension("parameter", serde_json::Value::String(param.clone())),

            ApiError::InvalidHeader { header, message } => ProblemDetails::new(
                "https://example.com/problems/invalid-header",
                "Invalid Header",
                400,
            )
            .with_detail(&format!("Invalid header '{}': {}", header, message))
            .with_extension("header", serde_json::Value::String(header.clone())),

            ApiError::MalformedRequest { message } => ProblemDetails::new(
                "https://example.com/problems/malformed-request",
                "Malformed Request",
                400,
            )
            .with_detail(message),
        }
    }

    /// Get HTTP status code
    pub fn status_code(&self) -> u16 {
        self.to_problem_details().status
    }
}

/// Helper trait for converting errors to ApiError
pub trait IntoApiError {
    fn into_api_error(self) -> ApiError;
}

// For form parsing errors (if using forms)
impl From<serde_urlencoded::de::Error> for ApiError {
    fn from(err: serde_urlencoded::de::Error) -> Self {
        ApiError::InvalidRequestBody {
            expected: "application/x-www-form-urlencoded".to_string(),
            message: err.to_string(),
        }
    }
}

impl axum::response::IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let problem_details = self.to_problem_details();
        let status = axum::http::StatusCode::from_u16(problem_details.status)
            .unwrap_or(axum::http::StatusCode::INTERNAL_SERVER_ERROR);

        let json = match serde_json::to_string(&problem_details) {
            Ok(json) => json,
            Err(_) => {
                // Fallback error response
                r#"{"type":"https://example.com/problems/internal-error","title":"Internal Server Error","status":500}"#.to_string()
            }
        };

        (
            status,
            [(
                axum::http::header::CONTENT_TYPE,
                ProblemDetails::CONTENT_TYPE,
            )],
            json,
        )
            .into_response()
    }
}

pub type ApiResult<T> = Result<T, ApiError>;

// Helper functions for common error scenarios
impl ApiError {
    /// Create an invalid request body error
    pub fn invalid_request_body(expected: impl Into<String>, message: impl Into<String>) -> Self {
        Self::InvalidRequestBody {
            expected: expected.into(),
            message: message.into(),
        }
    }

    /// Create an unauthorized error
    pub fn unauthorized(reason: Option<impl Into<String>>) -> Self {
        if let Some(reason) = reason {
            Self::Unauthorized {
                reason: reason.into(),
            }
        } else {
            Self::Unauthorized {
                reason: "You are not allowed to preform this action".to_string(),
            }
        }
    }

    /// Create a missing field error
    pub fn missing_field(field: impl Into<String>) -> Self {
        Self::MissingField {
            field: field.into(),
        }
    }

    /// Create an invalid content type error
    pub fn invalid_content_type(expected: impl Into<String>, received: impl Into<String>) -> Self {
        Self::InvalidContentType {
            expected: expected.into(),
            received: received.into(),
        }
    }

    /// Create a request too large error
    pub fn request_too_large(max_size: usize, received_size: usize) -> Self {
        Self::RequestTooLarge {
            max_size,
            received_size,
        }
    }
}

impl From<Infallible> for ApiError {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}
