use axum::{http::StatusCode, response::{IntoResponse, Response}};

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
            Error::TokenReused => (StatusCode::UNAUTHORIZED, "Token reuse detected, session revoked".into()),
            Error::TooManyAttempts => (StatusCode::TOO_MANY_REQUESTS, "Too many failed attempts".into()),
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

pub type Result<T> = std::result::Result<T, Error>;