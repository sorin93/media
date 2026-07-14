use axum::{
    async_trait,
    extract::{FromRequestParts, RequestPartsBorrow},
    http::{StatusCode, request::Parts},
};

use crate::auth::model::Authenticated;
use crate::auth::service::validate_access_token;
use crate::state::AppState;

pub struct OptionalAuth(pub Option<Authenticated>);

#[async_trait]
impl<S> FromRequestParts<S> for OptionalAuth
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts.headers.get("Authorization");

        if let Some(auth_value) = auth_header {
            if let Ok(auth_str) = auth_value.to_str() {
                if let Some(token) = auth_str.strip_prefix("Bearer ") {
                    let state = parts.extensions.get::<AppState>().cloned();

                    if let Some(state) = state {
                        if let Ok(authenticated) = validate_access_token(&state, token) {
                            return Ok(OptionalAuth(Some(authenticated)));
                        }
                    }
                }
            }
        }

        Ok(OptionalAuth(None))
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Authenticated
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts.headers.get("Authorization");

        if let Some(auth_value) = auth_header {
            if let Ok(auth_str) = auth_value.to_str() {
                if let Some(token) = auth_str.strip_prefix("Bearer ") {
                    let state = parts.extensions.get::<AppState>().cloned();

                    if let Some(state) = state {
                        if let Ok(authenticated) = validate_access_token(&state, token) {
                            return Ok(authenticated);
                        }
                    }
                }
            }
        }

        Err((StatusCode::UNAUTHORIZED, "Missing or invalid token".into()))
    }
}
