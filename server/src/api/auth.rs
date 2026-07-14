use axum::{extract::State, http::StatusCode, Json, middleware::from_fn_with_state, Router, routing::{get, post}};
use serde_json::{json, Value};

use crate::api::middleware;
use crate::models::auth::{Authenticated, LoginRequest, RefreshRequest};
use crate::error::Result;
use crate::models::user::User;
use crate::services::auth;
use crate::state::AppState;
use crate::services::util::{validate_email, validate_password};

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
    let (user, tokens) = auth::create(&state, input).await?;
    Ok((StatusCode::OK, Json(json!({
        "user": User::from(user),
        "tokens": tokens
    }))))
}

async fn refresh(
    State(state): State<AppState>,
    Json(input): Json<RefreshRequest>,
) -> Result<(StatusCode, Json<Value>)> {
    let (user, tokens) = auth::refresh(&state, input).await?;
    Ok((StatusCode::OK, Json(json!({
        "user": User::from(user),
        "tokens": tokens
    }))))
}

async fn remove(
    State(state): State<AppState>,
    auth: Authenticated,
) -> Result<()> {
    auth::remove(&state, auth.session_id).await?;
    Ok(())
}

async fn find(
    _: Authenticated,
) -> Result<()> {
    Ok(())
}