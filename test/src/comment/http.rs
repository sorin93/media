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
        .route("/media/{media_id}/comments", get(list))
        .route("/media/{media_id}/comments", post(create))
        .route("/comments/{comment_id}", delete(remove))
}

async fn list(
    State(state): State<AppState>,
    Path((media_id,)): Path<(Uuid,)>,
    Query(_query): Query<crate::comment::model::CommentParams>,
) -> Result<Json<Value>> {
    // TODO: Implement list comments
    Ok(Json(json!({})))
}

async fn create(
    State(state): State<AppState>,
    Path((media_id,)): Path<(Uuid,)>,
    Json(_input): Json<crate::comment::model::CreateComment>,
) -> Result<(StatusCode, Json<Value>)> {
    // TODO: Implement create comment
    Ok((StatusCode::CREATED, Json(json!({}))))
}

async fn remove(
    State(state): State<AppState>,
    Path((comment_id,)): Path<(Uuid,)>,
) -> Result<StatusCode> {
    // TODO: Implement remove comment
    Ok(StatusCode::NO_CONTENT)
}
