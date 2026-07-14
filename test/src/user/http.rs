use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post, put},
};
use serde_json::{Value, json};

use crate::core::validation::{validate_email, validate_password, validate_string};
use crate::error::{Error, Result};
use crate::state::AppState;
use crate::user::model::{RegisterRequest, UpdatePasswordRequest, User};
use crate::user::service;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/users/register", post(register))
        .route("/users/me", get(find_me))
        .route("/users/me/password", put(update_password))
}

async fn register(
    State(state): State<AppState>,
    Json(mut input): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<Value>)> {
    validate_string(&mut input.name, 3, 30, "Invalid name")?;
    validate_email(&mut input.email)?;
    validate_password(&mut input.password)?;

    // Check if user exists
    if crate::user::repository::exists_by_email(&state.pool, &input.email).await? {
        return Err(Error::UnprocessableEntity(
            "Email already registered".into(),
        ));
    }

    let password_hash = service::hash_password(&input.password)?;
    let user = crate::user::repository::create(
        &state.pool,
        &input.name,
        &input.email,
        &password_hash,
        input.phone,
        &input.country,
        input.birth_date,
    )
    .await?;

    Ok((
        StatusCode::CREATED,
        Json(json!({ "user": User::from(user) })),
    ))
}

async fn find_me(State(state): State<AppState>) -> Result<Json<Value>> {
    // TODO: Get authenticated user from request extensions
    Err(Error::Unauthorized("Not implemented".into()))
}

async fn update_password(
    State(state): State<AppState>,
    Json(input): Json<UpdatePasswordRequest>,
) -> Result<()> {
    // TODO: Get authenticated user from request extensions
    Err(Error::Unauthorized("Not implemented".into()))
}
