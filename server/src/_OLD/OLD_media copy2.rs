use axum::{extract::{Query, Path, multipart::Multipart, State}, http::StatusCode, Json, Router, routing::{delete, get, patch, post}};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::models::auth::Authenticated;
use crate::models::media::{CreateMedia, MediaQuery, MediaType::Image};
use crate::services::image::decode_multipart;
use crate::services::media;
use crate::services::util::{validate_number_opt, validate_string_opt};
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/media", get(list))
        .route("/media", post(create))
        .route("/media/{media_id}", patch(update))
        .route("/media/{media_id}", delete(remove))
}

async fn list(
    State(state): State<AppState>,
    auth: Authenticated,
    Query(input): Query<MediaQuery>,
) -> Result<Json<Value>> {
    validate_number_opt(input.page, 1, 10, "Invalid page")?;
    let media = media::list(&state, auth.user_id, input.search, input.page.unwrap_or(1)).await?;
    Ok(Json(json!({ "media": media })))
}

async fn create(
    State(state): State<AppState>,
    auth: Authenticated,
    multipart: Multipart,
) -> Result<(StatusCode, Json<Value>)> {
    let (data, image) = decode_multipart(multipart).await?;
    let mut input: CreateMedia = serde_json::from_value(data)?;
    validate_string_opt(&mut input.caption, 2, 100, "Invalid caption")?;
    if input.r#type != Image {
        return Err(Error::BadRequest("Invalid media type".into()));
    }
    let media = media::create(&state, auth.user_id, input.parent_media_id, input.r#type, input.caption, image).await?;
    Ok((StatusCode::CREATED, Json(json!({ "media": media }))))
}

async fn update(
    State(state): State<AppState>,
    auth: Authenticated,
    Path((media_id,)): Path<(Uuid,)>,
    Json(mut input): Json<CreateMedia>,
) -> Result<Json<Value>> {
    validate_string_opt(&mut input.caption, 2, 100, "Invalid caption")?;
    let media = media::update(&state, auth.user_id, media_id, input.caption).await?;
    Ok(Json(json!({ "media": media })))
}

async fn remove(
    State(state): State<AppState>,
    auth: Authenticated,
    Path((media_id,)): Path<(Uuid,)>,
) -> Result<StatusCode> {
    media::remove(&state, auth.user_id, media_id).await?;
    Ok(StatusCode::NO_CONTENT)
}