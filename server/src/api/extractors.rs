use axum::{extract::FromRequestParts, http::{header::AUTHORIZATION, request::Parts}};

use crate::{error::Error, models::auth::{Authenticated, OptionalAuth}, services, state::AppState};

impl<S> FromRequestParts<S> for Authenticated
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let app_state = parts.extensions.get::<AppState>().cloned()
            .ok_or_else(|| Error::Internal("AppState missing".into()))?;

        let token = parts.headers.get(AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "))
            .ok_or_else(|| Error::Unauthorized("Missing auth header".into()))?;

        services::auth::validate_access_token(&app_state, token)
    }
}

impl<S> FromRequestParts<S> for OptionalAuth
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let Some(app_state) = parts.extensions.get::<AppState>().cloned() else {
            return Ok(OptionalAuth(None));
        };

        let token = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "));

        match token {
            Some(token) => {
                let auth = services::auth::validate_access_token(&app_state, token)?;
                Ok(OptionalAuth(Some(auth)))
            }
            None => Ok(OptionalAuth(None)),
        }
    }
}