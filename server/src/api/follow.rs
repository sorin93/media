use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post},
};
use serde_json::{Value, json};
use uuid::Uuid;

use crate::error::Result;
use crate::models::auth::Authenticated;
use crate::models::util::Pagination;
use crate::services::follow;
use crate::services::util::validate_number_opt;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/followers", get(list))
        .route("/followers/{user_id}", post(create))
        .route("/followers/{user_id}", delete(remove))
}

// Get followers
async fn list(
    State(state): State<AppState>,
    auth: Authenticated,
    Query(input): Query<Pagination>,
) -> Result<Json<Value>> {
    validate_number_opt(input.page, 1, 10, "Invalid page")?;
    let users = follow::list(&state, auth.user_id, input.page.unwrap_or(1)).await?;
    Ok(Json(json!({ "users": users })))
}

// Follow
async fn create(
    State(state): State<AppState>,
    auth: Authenticated,
    Path((user_id,)): Path<(Uuid,)>,
) -> Result<(StatusCode, Json<Value>)> {
    let user = follow::create(&state, auth.user_id, user_id).await?;
    Ok((StatusCode::CREATED, Json(json!({ "user": user }))))
}

// Unfollow
async fn remove(
    State(state): State<AppState>,
    auth: Authenticated,
    Path((user_id,)): Path<(Uuid,)>,
) -> Result<StatusCode> {
    follow::remove(&state, auth.user_id, user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
