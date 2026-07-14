use axum::Json;
use pgvector::Vector;
use serde_json::{Map, Value, json};
use uuid::Uuid;

use crate::clients::google_cloud::get_embedding;
use crate::error::{Error, Result};
use crate::media::model::{Media, MediaType, Search};
use crate::media::repository;
use crate::state::AppState;

pub async fn explore(state: &AppState, page: i16) -> Result<Json<Value>> {
    let mut map = Map::new();
    let media = repository::list_explore(&state.pool, page).await?;
    map.insert("media".to_string(), json!(media));
    map.retain(|_, v| v.as_array().map_or(true, |a| !a.is_empty()));
    Ok(Json(Value::Object(map)))
}

pub async fn find_by_id(state: &AppState, media_id: Uuid) -> Result<Json<Value>> {
    let media = repository::find_by_id(&state.pool, media_id).await?;
    Ok(Json(json!({ "media": media })))
}

pub async fn create(
    state: &AppState,
    user_id: Uuid,
    parent_media_id: Option<Uuid>,
    r#type: MediaType,
    caption: Option<String>,
) -> Result<Uuid> {
    let media_id =
        repository::create(&state.pool, user_id, parent_media_id, r#type, caption).await?;
    Ok(media_id)
}

pub async fn activate(
    state: &AppState,
    user_id: Uuid,
    media_id: Uuid,
    embedding: Vector,
    blurhash: Option<String>,
) -> Result<Media> {
    let result = repository::activate(&state.pool, user_id, media_id, embedding, blurhash).await?;
    Ok(result)
}

pub async fn update(
    state: &AppState,
    user_id: Uuid,
    media_id: Uuid,
    caption: Option<String>,
) -> Result<Media> {
    let result = repository::update(&state.pool, user_id, media_id, caption).await?;
    Ok(result)
}

pub async fn remove(state: &AppState, user_id: Uuid, media_id: Uuid) -> Result<()> {
    repository::remove(&state.pool, user_id, media_id).await?;
    Ok(())
}
