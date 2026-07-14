use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post},
};
use serde_json::{Value, json};
use uuid::Uuid;

use crate::db::comment;
use crate::error::Result;
use crate::models::auth::Authenticated;
use crate::models::comment::CreateComment;
use crate::models::util::Pagination;
use crate::services::util::{validate_number_opt, validate_string};
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/comments/{media_id}", get(list))
        .route("/comments/{media_id}", post(create))
        .route("/comments/{media_id}/{comment_id}", delete(remove))
}

async fn list(
    State(state): State<AppState>,
    Path((media_id,)): Path<(Uuid,)>,
    Query(input): Query<Pagination>,
) -> Result<Json<Value>> {
    validate_number_opt(input.page, 1, 10, "Invalid page")?;
    let comments = comment::list(&state.pool, media_id, input.page.unwrap_or(1)).await?;
    Ok(Json(json!({ "comments": comments })))
}

async fn create(
    State(state): State<AppState>,
    auth: Authenticated,
    Path((media_id,)): Path<(Uuid,)>,
    Json(mut input): Json<CreateComment>,
) -> Result<(StatusCode, Json<Value>)> {
    validate_string(&mut input.text, 2, 1000, "Invalid comment")?;
    let comment = comment::create(&state.pool, auth.user_id, media_id, input.text).await?;
    Ok((StatusCode::CREATED, Json(json!({ "comment": comment }))))
}

async fn remove(
    State(state): State<AppState>,
    auth: Authenticated,
    Path((media_id, comment_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode> {
    comment::remove(&state.pool, auth.user_id, media_id, comment_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
