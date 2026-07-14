use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, patch, post},
};
use serde_json::{Value, json};
use uuid::Uuid;

use crate::core::validation::validate_number;
use crate::error::{Error, Result};
use crate::media::model::Pagination;
use crate::media::model::{CreateMedia, MediaType::Image, UpdateMedia};
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/explore", get(list))
        .route("/media/{media_id}", get(find_by_id))
        .route("/media", post(create))
        .route("/media/{media_id}", patch(update))
        .route("/media/{media_id}", delete(remove))
}

async fn list(
    State(state): State<AppState>,
    Query(query): Query<Pagination>,
) -> Result<Json<Value>> {
    let page = query.page.unwrap_or(1);
    validate_number(page, 1, 10, "Invalid page")?;
    let media = crate::media::repository::list_explore(&state.pool, page).await?;
    Ok(Json(json!({ "media": media })))
}

async fn find_by_id(
    State(state): State<AppState>,
    Path((media_id,)): Path<(Uuid,)>,
) -> Result<Json<Value>> {
    let media = crate::media::repository::find_by_id(&state.pool, media_id).await?;
    Ok(Json(json!({ "media": media })))
}

async fn create(
    State(state): State<AppState>,
    Json(mut input): Json<CreateMedia>,
) -> Result<(StatusCode, Json<Value>)> {
    // TODO: Get authenticated user from request extensions
    let user_id = uuid::Uuid::new_v4(); // Placeholder

    if input.r#type != Image {
        return Err(Error::BadRequest("Invalid media type".into()));
    }
    let media_id = crate::media::service::create(
        &state,
        user_id,
        input.parent_media_id,
        input.r#type,
        input.caption,
    )
    .await?;
    Ok((StatusCode::CREATED, Json(json!({ "media_id": media_id }))))
}

async fn update(
    State(state): State<AppState>,
    Path((media_id,)): Path<(Uuid,)>,
    Json(input): Json<UpdateMedia>,
) -> Result<Json<Value>> {
    // TODO: Get authenticated user from request extensions
    let user_id = uuid::Uuid::new_v4(); // Placeholder

    let media = crate::media::service::update(&state, user_id, media_id, input.caption).await?;
    Ok(Json(json!({ "media": media })))
}

async fn remove(
    State(state): State<AppState>,
    Path((media_id,)): Path<(Uuid,)>,
) -> Result<StatusCode> {
    // TODO: Get authenticated user from request extensions
    let user_id = uuid::Uuid::new_v4(); // Placeholder

    crate::media::service::remove(&state, user_id, media_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
