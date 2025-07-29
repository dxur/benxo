use std::{borrow::Cow, collections::HashMap, convert::Infallible};

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
    ValidationError {
        field: Cow<'static, str>,
        message: Cow<'static, str>,
    },

    #[error("Resource not found")]
    NotFound {
        resource: Cow<'static, str>,
        id: Cow<'static, str>,
    },

    #[error("Authentication failed")]
    Unauthorized { reason: Cow<'static, str> },

    #[error("Access denied")]
    Forbidden {
        resource: Cow<'static, str>,
        action: Cow<'static, str>,
    },

    #[error("Conflict occurred")]
    Conflict {
        resource: Cow<'static, str>,
        reason: Cow<'static, str>,
    },

    #[error("Rate limit exceeded")]
    RateLimitExceeded { retry_after: u64 },

    #[error("Internal server error")]
    InternalError { message: Cow<'static, str> },

    #[error("Service unavailable")]
    ServiceUnavailable {
        service: Cow<'static, str>,
        retry_after: Option<u64>,
    },

    #[error("Database error: {0}")]
    DatabaseError(Cow<'static, str>),

    #[error("External service error: {0}")]
    ExternalServiceError(Cow<'static, str>),

    #[error("Invalid JSON syntax")]
    InvalidJson {
        message: Cow<'static, str>,
        line: Option<usize>,
        column: Option<usize>,
    },

    #[error("Invalid request body")]
    InvalidRequestBody {
        expected: Cow<'static, str>,
        message: Cow<'static, str>,
    },

    #[error("Missing required field")]
    MissingField { field: Cow<'static, str> },

    #[error("Invalid content type")]
    InvalidContentType {
        expected: Cow<'static, str>,
        received: Cow<'static, str>,
    },

    #[error("Request body too large")]
    RequestTooLarge {
        max_size: usize,
        received_size: usize,
    },

    #[error("Invalid query parameter")]
    InvalidQueryParam {
        param: Cow<'static, str>,
        message: Cow<'static, str>,
    },

    #[error("Invalid path parameter")]
    InvalidPathParam {
        param: Cow<'static, str>,
        message: Cow<'static, str>,
    },

    #[error("Invalid header")]
    InvalidHeader {
        header: Cow<'static, str>,
        message: Cow<'static, str>,
    },

    #[error("Malformed request")]
    MalformedRequest { message: Cow<'static, str> },
}

impl ApiError {
    // === Validation Errors ===

    /// Create a validation error with static strings
    pub fn validation(
        field: impl Into<Cow<'static, str>>,
        message: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self::ValidationError {
            field: field.into(),
            message: message.into(),
        }
    }

    /// Create a validation error for a required field
    pub fn required_field(field: impl Into<Cow<'static, str>>) -> Self {
        let field = field.into();
        Self::ValidationError {
            message: format!("Field '{}' is required", field).into(),
            field,
        }
    }

    /// Create a validation error for invalid field format
    pub fn invalid_format(field: impl Into<Cow<'static, str>>, expected: &'static str) -> Self {
        let field = field.into();
        Self::ValidationError {
            message: format!(
                "Field '{}' has invalid format. Expected: {}",
                field, expected
            )
            .into(),
            field,
        }
    }

    /// Create a validation error for field length
    pub fn invalid_length(
        field: impl Into<Cow<'static, str>>,
        min: Option<usize>,
        max: Option<usize>,
    ) -> Self {
        let field = field.into();
        let message = match (min, max) {
            (Some(min), Some(max)) => format!(
                "Field '{}' must be between {} and {} characters",
                field, min, max
            ),
            (Some(min), None) => format!("Field '{}' must be at least {} characters", field, min),
            (None, Some(max)) => format!("Field '{}' must be at most {} characters", field, max),
            (None, None) => format!("Field '{}' has invalid length", field),
        };

        Self::ValidationError {
            message: message.into(),
            field,
        }
    }

    // === Resource Errors ===

    /// Create a not found error
    pub fn not_found(
        resource: impl Into<Cow<'static, str>>,
        id: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self::NotFound {
            resource: resource.into(),
            id: id.into(),
        }
    }

    // === Authentication & Authorization ===

    /// Create an unauthorized error
    pub fn unauthorized(reason: impl Into<Cow<'static, str>>) -> Self {
        Self::Unauthorized {
            reason: reason.into(),
        }
    }

    /// Create a token expired error
    pub fn token_expired() -> Self {
        Self::unauthorized("Token has expired")
    }

    /// Create an invalid token error
    pub fn invalid_token() -> Self {
        Self::unauthorized("Invalid or malformed token")
    }

    /// Create a missing token error
    pub fn missing_token() -> Self {
        Self::unauthorized("Authentication token is required")
    }

    /// Create a forbidden error
    pub fn forbidden(
        resource: impl Into<Cow<'static, str>>,
        action: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self::Forbidden {
            resource: resource.into(),
            action: action.into(),
        }
    }

    /// Create a permission denied error
    pub fn permission_denied(action: &'static str) -> Self {
        Self::forbidden("Resource", action)
    }

    // === Request Format Errors ===

    /// Create an invalid JSON error
    pub fn invalid_json(message: impl Into<Cow<'static, str>>) -> Self {
        Self::InvalidJson {
            message: message.into(),
            line: None,
            column: None,
        }
    }

    /// Create an invalid JSON error with position
    pub fn invalid_json_at(
        message: impl Into<Cow<'static, str>>,
        line: usize,
        column: usize,
    ) -> Self {
        Self::InvalidJson {
            message: message.into(),
            line: Some(line),
            column: Some(column),
        }
    }

    /// Create an invalid request body error
    pub fn invalid_body(
        expected: impl Into<Cow<'static, str>>,
        message: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self::InvalidRequestBody {
            expected: expected.into(),
            message: message.into(),
        }
    }

    /// Create a missing field error
    pub fn missing_field(field: impl Into<Cow<'static, str>>) -> Self {
        Self::MissingField {
            field: field.into(),
        }
    }

    /// Create an invalid content type error
    pub fn invalid_content_type(
        expected: impl Into<Cow<'static, str>>,
        received: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self::InvalidContentType {
            expected: expected.into(),
            received: received.into(),
        }
    }

    /// Create a JSON content type expected error
    pub fn json_expected(received: impl Into<Cow<'static, str>>) -> Self {
        Self::invalid_content_type("application/json", received)
    }

    /// Create a request too large error
    pub fn request_too_large(max_size: usize, received_size: usize) -> Self {
        Self::RequestTooLarge {
            max_size,
            received_size,
        }
    }

    // === Parameter Errors ===

    /// Create an invalid query parameter error
    pub fn invalid_query(
        param: impl Into<Cow<'static, str>>,
        message: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self::InvalidQueryParam {
            param: param.into(),
            message: message.into(),
        }
    }

    /// Create an invalid query parameter type error
    pub fn invalid_query_type(
        param: impl Into<Cow<'static, str>>,
        expected_type: &'static str,
    ) -> Self {
        let param = param.into();
        Self::InvalidQueryParam {
            message: format!("Parameter '{}' must be of type {}", param, expected_type).into(),
            param,
        }
    }

    /// Create an invalid path parameter error
    pub fn invalid_path(
        param: impl Into<Cow<'static, str>>,
        message: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self::InvalidPathParam {
            param: param.into(),
            message: message.into(),
        }
    }

    /// Create an invalid UUID path parameter error
    pub fn invalid_uuid(param: impl Into<Cow<'static, str>>) -> Self {
        let param = param.into();
        Self::InvalidPathParam {
            message: format!("Parameter '{}' must be a valid UUID", param).into(),
            param,
        }
    }

    /// Create an invalid header error
    pub fn invalid_header(
        header: impl Into<Cow<'static, str>>,
        message: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self::InvalidHeader {
            header: header.into(),
            message: message.into(),
        }
    }

    /// Create a missing header error
    pub fn missing_header(header: impl Into<Cow<'static, str>>) -> Self {
        let header = header.into();
        Self::InvalidHeader {
            message: format!("Header '{}' is required", header).into(),
            header,
        }
    }

    // === Business Logic Errors ===

    /// Create a conflict error
    pub fn conflict(
        resource: impl Into<Cow<'static, str>>,
        reason: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self::Conflict {
            resource: resource.into(),
            reason: reason.into(),
        }
    }

    /// Create a duplicate resource error
    pub fn duplicate(resource: impl Into<Cow<'static, str>>, field: &'static str) -> Self {
        let resource = resource.into();
        Self::Conflict {
            reason: format!("{} with this {} already exists", resource, field).into(),
            resource,
        }
    }

    // === Rate Limiting ===

    /// Create a rate limit exceeded error
    pub fn rate_limited(retry_after: u64) -> Self {
        Self::RateLimitExceeded { retry_after }
    }

    // === Server Errors ===

    /// Create an internal error
    pub fn internal(message: impl Into<Cow<'static, str>>) -> Self {
        Self::InternalError {
            message: message.into(),
        }
    }

    /// Create a database error
    pub fn database(message: impl Into<Cow<'static, str>>) -> Self {
        Self::DatabaseError(message.into())
    }

    /// Create an external service error
    pub fn external_service(message: impl Into<Cow<'static, str>>) -> Self {
        Self::ExternalServiceError(message.into())
    }

    /// Create a service unavailable error
    pub fn service_unavailable(
        service: impl Into<Cow<'static, str>>,
        retry_after: Option<u64>,
    ) -> Self {
        Self::ServiceUnavailable {
            service: service.into(),
            retry_after,
        }
    }

    /// Create a malformed request error
    pub fn malformed(message: impl Into<Cow<'static, str>>) -> Self {
        Self::MalformedRequest {
            message: message.into(),
        }
    }

    // === Legacy helper functions (kept for compatibility) ===

    /// Create an invalid request body error (legacy)
    #[deprecated(since = "0.1.0", note = "Use `invalid_body` instead")]
    pub fn invalid_request_body(
        expected: impl Into<Cow<'static, str>>,
        message: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self::invalid_body(expected, message)
    }

    /// Convert ApiError to RFC 7807 ProblemDetails
    pub fn to_problem_details(&self) -> ProblemDetails {
        debug!("Returning error: {:?}", self);
        match self {
            ApiError::ValidationError { field, message } => {
                ProblemDetails::new("/docs/problems/validation-error", "Validation Error", 400)
                    .with_detail(message.as_ref())
                    .with_extension("field", serde_json::Value::String(field.to_string()))
            }

            ApiError::NotFound { resource, id } => {
                ProblemDetails::new("/docs/problems/not-found", "Resource Not Found", 404)
                    .with_detail(&format!("{} with id '{}' was not found", resource, id))
                    .with_extension("resource", serde_json::Value::String(resource.to_string()))
                    .with_extension("id", serde_json::Value::String(id.to_string()))
            }

            ApiError::Unauthorized { reason } => ProblemDetails::new(
                "/docs/problems/unauthorized",
                "Authentication Required",
                401,
            )
            .with_detail(reason.as_ref()),

            ApiError::Forbidden { resource, action } => {
                ProblemDetails::new("/docs/problems/forbidden", "Access Denied", 403)
                    .with_detail(&format!("Access denied for {} on {}", action, resource))
                    .with_extension("resource", serde_json::Value::String(resource.to_string()))
                    .with_extension("action", serde_json::Value::String(action.to_string()))
            }

            ApiError::Conflict { resource, reason } => {
                ProblemDetails::new("/docs/problems/conflict", "Resource Conflict", 409)
                    .with_detail(&format!("Conflict with {}: {}", resource, reason))
                    .with_extension("resource", serde_json::Value::String(resource.to_string()))
            }

            ApiError::RateLimitExceeded { retry_after } => ProblemDetails::new(
                "/docs/problems/rate-limit-exceeded",
                "Rate Limit Exceeded",
                429,
            )
            .with_detail("Too many requests. Please try again later.")
            .with_extension(
                "retry_after",
                serde_json::Value::Number((*retry_after).into()),
            ),

            ApiError::InternalError { message: _ } => ProblemDetails::new(
                "/docs/problems/internal-error",
                "Internal Server Error",
                500,
            )
            .with_detail("An unexpected error occurred")
            .with_extension(
                "error_id",
                serde_json::Value::String(ObjectId::new().to_hex()),
            ),

            ApiError::ServiceUnavailable {
                service,
                retry_after,
            } => {
                let mut problem = ProblemDetails::new(
                    "/docs/problems/service-unavailable",
                    "Service Unavailable",
                    503,
                )
                .with_detail(&format!(
                    "The {} service is temporarily unavailable",
                    service
                ))
                .with_extension("service", serde_json::Value::String(service.to_string()));

                if let Some(retry) = retry_after {
                    problem = problem
                        .with_extension("retry_after", serde_json::Value::Number((*retry).into()));
                }

                problem
            }

            ApiError::DatabaseError(_msg) => {
                ProblemDetails::new("/docs/problems/database-error", "Database Error", 500)
                    .with_detail("A database error occurred")
                    .with_extension(
                        "error_id",
                        serde_json::Value::String(ObjectId::new().to_hex()),
                    )
            }

            ApiError::ExternalServiceError(_msg) => ProblemDetails::new(
                "/docs/problems/external-service-error",
                "External Service Error",
                502,
            )
            .with_detail("An external service error occurred")
            .with_extension(
                "error_id",
                serde_json::Value::String(ObjectId::new().to_hex()),
            ),

            ApiError::InvalidJson {
                message,
                line,
                column,
            } => {
                let mut problem =
                    ProblemDetails::new("/docs/problems/invalid-json", "Invalid JSON Syntax", 400)
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
                "/docs/problems/invalid-request-body",
                "Invalid Request Body",
                400,
            )
            .with_detail(&format!("Request body validation failed: {}", message))
            .with_extension(
                "expected_format",
                serde_json::Value::String(expected.to_string()),
            ),

            ApiError::MissingField { field } => ProblemDetails::new(
                "/docs/problems/missing-field",
                "Missing Required Field",
                400,
            )
            .with_detail(&format!("Required field '{}' is missing", field))
            .with_extension("field", serde_json::Value::String(field.to_string())),

            ApiError::InvalidContentType { expected, received } => ProblemDetails::new(
                "/docs/problems/invalid-content-type",
                "Invalid Content Type",
                415,
            )
            .with_detail(&format!(
                "Expected '{}' but received '{}'",
                expected, received
            ))
            .with_extension("expected", serde_json::Value::String(expected.to_string()))
            .with_extension("received", serde_json::Value::String(received.to_string())),

            ApiError::RequestTooLarge {
                max_size,
                received_size,
            } => ProblemDetails::new(
                "/docs/problems/request-too-large",
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
                "/docs/problems/invalid-query-param",
                "Invalid Query Parameter",
                400,
            )
            .with_detail(&format!("Invalid query parameter '{}': {}", param, message))
            .with_extension("parameter", serde_json::Value::String(param.to_string())),

            ApiError::InvalidPathParam { param, message } => ProblemDetails::new(
                "/docs/problems/invalid-path-param",
                "Invalid Path Parameter",
                400,
            )
            .with_detail(&format!("Invalid path parameter '{}': {}", param, message))
            .with_extension("parameter", serde_json::Value::String(param.to_string())),

            ApiError::InvalidHeader { header, message } => {
                ProblemDetails::new("/docs/problems/invalid-header", "Invalid Header", 400)
                    .with_detail(&format!("Invalid header '{}': {}", header, message))
                    .with_extension("header", serde_json::Value::String(header.to_string()))
            }

            ApiError::MalformedRequest { message } => {
                ProblemDetails::new("/docs/problems/malformed-request", "Malformed Request", 400)
                    .with_detail(message.as_ref())
            }
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
        ApiError::invalid_body("application/x-www-form-urlencoded", err.to_string())
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
                r#"{"type":"/docs/problems/internal-error","title":"Internal Server Error","status":500}"#.to_string()
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

impl From<Infallible> for ApiError {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}
