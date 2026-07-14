use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::fmt;

#[derive(Debug)]
pub enum Error {
    BadRequest(String),
    Conflict(String),
    Internal(String),
    NotFound(String),
    PayloadTooLarge(String),
    Unauthorized(String),
    UnprocessableEntity(String),
    CodeExpired,
    CodeInvalid,
    TokenExpired,
    TokenInvalid,
    TokenReused,
    TooManyAttempts,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Error::BadRequest(message) => (StatusCode::BAD_REQUEST, message),
            Error::Conflict(message) => (StatusCode::CONFLICT, message),
            Error::Internal(message) => (StatusCode::INTERNAL_SERVER_ERROR, message),
            Error::NotFound(message) => (StatusCode::NOT_FOUND, message),
            Error::PayloadTooLarge(message) => (StatusCode::PAYLOAD_TOO_LARGE, message),
            Error::Unauthorized(message) => (StatusCode::UNAUTHORIZED, message),
            Error::UnprocessableEntity(message) => (StatusCode::UNPROCESSABLE_ENTITY, message),
            Error::CodeExpired => (StatusCode::BAD_REQUEST, "Verification code expired".into()),
            Error::CodeInvalid => (StatusCode::BAD_REQUEST, "Invalid verification code".into()),
            Error::TokenExpired => (StatusCode::UNAUTHORIZED, "Token has expired".into()),
            Error::TokenInvalid => (StatusCode::UNAUTHORIZED, "Token is invalid".into()),
            Error::TokenReused => (
                StatusCode::UNAUTHORIZED,
                "Token reuse detected, session revoked".into(),
            ),
            Error::TooManyAttempts => (
                StatusCode::TOO_MANY_REQUESTS,
                "Too many failed attempts".into(),
            ),
        };

        (status, message).into_response()
    }
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        tracing::error!("Database error: {:?}", e);
        match e {
            sqlx::Error::RowNotFound => Error::NotFound("Resource not found".into()),
            sqlx::Error::Database(ref db_err) if db_err.is_unique_violation() => {
                Error::Conflict("Resource already exists".into())
            }
            _ => Error::Internal("Database error".into()),
        }
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        use jsonwebtoken::errors::ErrorKind;
        match e.kind() {
            ErrorKind::ExpiredSignature => Error::TokenExpired,
            _ => Error::TokenInvalid,
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        tracing::error!("JSON parse error: {:?}", e);
        Error::UnprocessableEntity(format!("Invalid JSON: {}", e))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Error::BadRequest(msg) => msg,
            Error::Conflict(msg) => msg,
            Error::Internal(msg) => msg,
            Error::NotFound(msg) => msg,
            Error::PayloadTooLarge(msg) => msg,
            Error::Unauthorized(msg) => msg,
            Error::UnprocessableEntity(msg) => msg,
            Error::CodeExpired => "Verification code expired",
            Error::CodeInvalid => "Invalid verification code",
            Error::TokenExpired => "Token has expired",
            Error::TokenInvalid => "Token is invalid",
            Error::TokenReused => "Token reuse detected, session revoked",
            Error::TooManyAttempts => "Too many failed attempts",
        };
        write!(f, "{}", message)
    }
}

impl From<sqlx::migrate::MigrateError> for Error {
    fn from(e: sqlx::migrate::MigrateError) -> Self {
        tracing::error!("Migration error: {:?}", e);
        Error::Internal("Database migration failed".into())
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
