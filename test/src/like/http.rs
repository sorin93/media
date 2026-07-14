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
        .route("/media/{media_id}/likes", get(list))
        .route("/media/{media_id}/like", post(like))
        .route("/media/{media_id}/like", delete(unlike))
}

async fn list(
    State(state): State<AppState>,
    Path((media_id,)): Path<(Uuid,)>,
    Query(_query): Query<crate::like::model::LikeParams>,
) -> Result<Json<Value>> {
    // TODO: Implement list likes
    Ok(Json(json!({})))
}

async fn like(
    State(state): State<AppState>,
    Path((media_id,)): Path<(Uuid,)>,
) -> Result<StatusCode> {
    // TODO: Get authenticated user from request extensions
    let user_id = uuid::Uuid::new_v4(); // Placeholder
    Ok(StatusCode::CREATED)
}

async fn unlike(
    State(state): State<AppState>,
    Path((media_id,)): Path<(Uuid,)>,
) -> Result<StatusCode> {
    // TODO: Get authenticated user from request extensions
    let user_id = uuid::Uuid::new_v4(); // Placeholder
    Ok(StatusCode::NO_CONTENT)
}
