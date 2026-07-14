use axum::{extract::{Query, Path, State}, http::StatusCode, Json, middleware::from_fn_with_state, Router, routing::{delete, get, post, put}};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::api::middleware;
use crate::error::Result;
use crate::models::auth::{Authenticated, OptionalAuth};
use crate::models::user::{RegisterRequest, UpdatePasswordRequest, User};
use crate::models::util::Pagination;
use crate::services::{comment, like, media, user};
use crate::services::util::{validate_country, validate_email, validate_number, validate_password, validate_string, validate_string_opt};
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(home))  // // todo: optional auth demo remove
        .route("/users", post(create))
        .route_layer(from_fn_with_state(AppState::clone, middleware::guest_only))
        .route("/users/me", get(find))
        .route("/users/me", delete(remove))
        .route("/users/me/password", put(update_password))
        .route("/users/{user_id}", get(find_user_by_id))
        .route("/users/{user_id}/media", get(user_media))
        .route("/users/{user_id}/comments", get(user_comments))
        .route("/users/{user_id}/likes", get(user_likes))
}

async fn find(
    State(state): State<AppState>,
    auth: Authenticated,
) -> Result<Json<User>> {
    let user = user::find_account_by_id(&state, auth.user_id).await?;
    Ok(Json(user.into()))
}

async fn create(
    State(state): State<AppState>,
    Json(mut input): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<Value>)> {
    if input.code.is_none() {
        user::initiate_registration(&state, input).await?;
        Ok((StatusCode::OK, Json(json!({}))))
    } else {
        validate_string(&mut input.name, 2, 50, "Invalid name")?;
        validate_email(&mut input.email)?;
        validate_password(&mut input.password)?;
        validate_string_opt(&mut input.phone, 10, 15, "Invalid phone")?;
        validate_country(&mut input.country)?;
        // todo: validate birth-date
        let (user, tokens) = user::complete_registration(&state, input).await?;
        Ok((StatusCode::CREATED, Json(json!({
            "user": User::from(user),
            "tokens": tokens,
        }))))
    }
}

async fn update_password(
    State(state): State<AppState>,
    auth: Authenticated,
    Json(mut input): Json<UpdatePasswordRequest>,
) -> Result<Json<User>> {
    validate_password(&mut input.current_password)?;
    validate_password(&mut input.new_password)?;
    let user = user::update_password(&state, auth.user_id, &input.current_password, &input.new_password).await?;
    Ok(Json(user.into()))
}

async fn remove(
    State(state): State<AppState>,
    auth: Authenticated,
) -> Result<Json<serde_json::Value>> {
    user::remove(&state, auth.user_id).await?;
    Ok(Json(serde_json::json!({ "message": "Account deleted" })))
}

// todo: optional auth demo remove
async fn home(OptionalAuth(_auth): OptionalAuth) -> StatusCode {
    StatusCode::OK
}

async fn user_media(
    State(state): State<AppState>,
    Path((user_id,)): Path<(Uuid,)>,
    Query(query): Query<Pagination>,
) -> Result<Json<Value>> {
    let page = query.page.unwrap_or(1);
    let expand = query.expand.unwrap_or(false);
    validate_number(page, 1, 10, "Invalid page")?;
    let json = media::user_media_by_user_id(&state, user_id, page, expand).await?;
    Ok(json)
}

async fn user_comments(
    State(state): State<AppState>,
    Path((user_id,)): Path<(Uuid,)>,
    Query(query): Query<Pagination>,
) -> Result<Json<Value>> {
    let page = query.page.unwrap_or(1);
    let expand = query.expand.unwrap_or(false);
    validate_number(page, 1, 10, "Invalid page")?;
    let json = comment::user_comments(&state, user_id, page, expand).await?;
    Ok(json)
}

async fn user_likes(
    State(state): State<AppState>,
    Path((user_id,)): Path<(Uuid,)>,
    Query(query): Query<Pagination>,
) -> Result<Json<Value>> {
    let page = query.page.unwrap_or(1);
    let expand = query.expand.unwrap_or(false);
    validate_number(page, 1, 10, "Invalid page")?;
    let json = like::user_likes(&state, user_id, page, expand).await?;
    Ok(json)
}

async fn find_user_by_id(
    State(state): State<AppState>,
    Path((user_id,)): Path<(Uuid,)>,
) -> Result<Json<Value>> {
    let json = user::find_user_by_id(&state, user_id).await?;
    Ok(json)
}