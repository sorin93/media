use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post},
};
use serde_json::{Value, json};
use uuid::Uuid;

use crate::error::Result;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/users/{user_id}/follow", post(follow))
        .route("/users/{user_id}/follow", delete(unfollow))
        .route("/users/{user_id}/followers", get(list_followers))
        .route("/users/{user_id}/following", get(list_following))
}

async fn follow(
    State(state): State<AppState>,
    Path((target_user_id,)): Path<(Uuid,)>,
) -> Result<StatusCode> {
    // TODO: Get authenticated user from request extensions
    let user_id = uuid::Uuid::new_v4(); // Placeholder
    // TODO: Implement follow logic
    Ok(StatusCode::CREATED)
}

async fn unfollow(
    State(state): State<AppState>,
    Path((target_user_id,)): Path<(Uuid,)>,
) -> Result<StatusCode> {
    // TODO: Get authenticated user from request extensions
    let user_id = uuid::Uuid::new_v4(); // Placeholder
    // TODO: Implement unfollow logic
    Ok(StatusCode::NO_CONTENT)
}

async fn list_followers(
    State(state): State<AppState>,
    Path((user_id,)): Path<(Uuid,)>,
    Query(_query): Query<crate::follow::model::FollowParams>,
) -> Result<Json<Value>> {
    // TODO: Implement list followers
    Ok(Json(json!({})))
}

async fn list_following(
    State(state): State<AppState>,
    Path((user_id,)): Path<(Uuid,)>,
    Query(_query): Query<crate::follow::model::FollowParams>,
) -> Result<Json<Value>> {
    // TODO: Implement list following
    Ok(Json(json!({})))
}
