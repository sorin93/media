use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    middleware::from_fn_with_state,
    routing::{get, post},
};
use serde_json::{Value, json};

use crate::auth::middleware;
use crate::auth::model::{LoginRequest, RefreshRequest};
use crate::auth::service;
use crate::core::validation::{validate_email, validate_password};
use crate::error::Result;
use crate::state::AppState;
use crate::user::model::User;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/auth/login", post(create))
        .route_layer(from_fn_with_state(AppState::clone, middleware::guest_only))
        .route("/auth/refresh", post(refresh))
        .route("/auth/logout", post(remove))
        .route("/auth/me", get(find))
}

async fn create(
    State(state): State<AppState>,
    Json(mut input): Json<LoginRequest>,
) -> Result<(StatusCode, Json<Value>)> {
    validate_email(&mut input.email)?;
    validate_password(&mut input.password)?;
    let (user, tokens) = service::create(&state, input).await?;
    Ok((
        StatusCode::OK,
        Json(json!({
            "user": User::from(user),
            "tokens": tokens
        })),
    ))
}

async fn refresh(
    State(state): State<AppState>,
    Json(input): Json<RefreshRequest>,
) -> Result<(StatusCode, Json<Value>)> {
    let (user, tokens) = service::refresh(&state, input).await?;
    Ok((
        StatusCode::OK,
        Json(json!({
            "user": User::from(user),
            "tokens": tokens
        })),
    ))
}

async fn remove(State(state): State<AppState>) -> Result<()> {
    // TODO: Get user from request extensions (set by middleware)
    // For now, return an error
    Err(crate::error::Error::Unauthorized("Not implemented".into()))
}

async fn find(State(state): State<AppState>) -> Result<Json<Value>> {
    // TODO: Get user from request extensions (set by middleware)
    // For now, return an error
    Err(crate::error::Error::Unauthorized("Not implemented".into()))
}
