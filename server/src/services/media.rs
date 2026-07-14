use axum::Json;
use pgvector::Vector;
use serde_json::{json, Map, Value};
use uuid::Uuid;

use crate::clients::google_cloud::get_embedding;
use crate::db::{comment, history, media, text, user};
use crate::error::{Error, Result};
use crate::models::history::HistoryType;
use crate::models::media::{Media, MediaType, Search};
use crate::services::util::validate_string;
use crate::state::AppState;

pub async fn explore(
    state: &AppState,
    page: i16,
) -> Result<Json<Value>> {
    let mut map = Map::new();
    let media = media::list_explore(&state.pool, page).await?;
    map.insert("media".to_string(), json!(media));
    // toto: add texts
    map.retain(|_, v| v.as_array().map_or(true, |a| !a.is_empty()));
    Ok(Json(Value::Object(map)))
}

pub async fn following(
    state: &AppState,
    user_id: Uuid,
    page: i16,
) -> Result<Json<Value>> {
    let mut map = Map::new();
    let media = media::list_following(&state.pool, user_id, page).await?;
    map.insert("media".to_string(), json!(media));
    map.retain(|_, v| v.as_array().map_or(true, |a| !a.is_empty()));
    Ok(Json(Value::Object(map)))
}

pub async fn search(
    state: &AppState,
    user_id: Option<Uuid>,
    mut text: String,
    page: i16,
) -> Result<Json<Value>> {
    validate_string(&mut text, 3, 100, "Invalid query")?;
    let text = text.to_lowercase();
    let text = get_embedding(&state, text.clone()).await?;
    if let Some(user_id) = user_id {
        history::create(&state.pool, user_id, text.text_id, HistoryType::Text).await?;
    }
    let mut map = Map::new();
    let media = media::list_similar_by_embedding(&state.pool, text.embedding.clone(), page).await?;
    let texts = text::list_by_embedding(&state.pool, text.embedding).await?;
    map.insert("media".to_string(), json!(media));
    map.insert("texts".to_string(), json!(texts));
    map.retain(|_, v| v.as_array().map_or(true, |a| !a.is_empty()));
    Ok(Json(Value::Object(map)))
}

pub async fn user_media_by_user_id(
    state: &AppState,
    user_id: Uuid,
    page: i16,
    expand: bool,
) -> Result<Json<Value>> {
    let mut map = Map::new();
    if expand {
        let subject = user::find_user_by_id(&state.pool, user_id).await?;
        map.insert("subject".to_string(), json!(subject));
    }
    let media = media::user_media(&state.pool, user_id, page).await?;
    map.insert("media".to_string(), json!(media));
    map.retain(|_, v| v.as_array().map_or(true, |a| !a.is_empty()));
    Ok(Json(Value::Object(map)))
}

pub async fn user_media_by_media_id(
    state: &AppState,
    media_id: Uuid,
    page: i16,
    expand: bool,
) -> Result<Json<Value>> {
    let media = media::find_by_id(&state.pool, media_id).await?;
    let json = user_media_by_user_id(state, media.user.user_id, page, expand).await?;
    Ok(json)
}

pub async fn suggestions(
    state: &AppState,
    media_id: Uuid,
    page: i16,
    expand: bool,
) -> Result<Json<Value>> {
    let mut map = Map::new();
    if expand {
        let subject = media::find_by_id(&state.pool, media_id).await?;
        map.insert("subject".to_string(), json!(subject));
    }
    let media = media::list_similar_by_id(&state.pool, media_id, page).await?;
    let texts = text::list_by_id(&state.pool, media_id).await?;
    map.insert("media".to_string(), json!(media));
    map.insert("texts".to_string(), json!(texts));
    map.retain(|_, v| v.as_array().map_or(true, |a| !a.is_empty()));
    Ok(Json(Value::Object(map)))
}

pub async fn list(
    state: &AppState,
    user_id: Option<Uuid>,
    mut search: Search,
    page: i16,
) -> Result<Json<Value>> {
    let mut map = Map::new();
    match search {
        Search::User(user_id) => {
            let user = user::find_user_by_id(&state.pool, user_id).await?;
            let media = media::list_user(&state.pool, user_id, page).await?;
            map.insert("user".to_string(), json!(user));
            map.insert("media".to_string(), json!(media));
        }
        Search::Liked(user_id) => {
            let media = media::list_liked(&state.pool, user_id, page).await?;
            map.insert("media".to_string(), json!(media));
        }
        Search::Replied(user_id) => {
            let media = media::list_replied(&state.pool, user_id, page).await?;
            map.insert("media".to_string(), json!(media));
        }
        Search::Following => {
            let user_id = user_id.ok_or(Error::Unauthorized("Unauthorized".into()))?;
            let media = media::list_following(&state.pool, user_id, page).await?;
            map.insert("media".to_string(), json!(media));
        }
        Search::Media(media_id) => {
            let media = media::list_similar_by_id(&state.pool, media_id, page).await?;
            let texts = text::list_by_id(&state.pool, media_id).await?;
            map.insert("media".to_string(), json!(media));
            map.insert("texts".to_string(), json!(texts));
        }
        Search::Text(ref mut text) => {
            validate_string(text, 3, 100, "Invalid query")?;
            *text = text.to_lowercase();
            let text = get_embedding(&state, text.clone()).await?;
            if let Some(user_id) = user_id {
                history::create(&state.pool, user_id, text.text_id, HistoryType::Text).await?;
            }
            let media = media::list_similar_by_embedding(&state.pool, text.embedding.clone(), page).await?;
            let texts = text::list_by_embedding(&state.pool, text.embedding).await?;
            map.insert("media".to_string(), json!(media));
            map.insert("texts".to_string(), json!(texts/*.split_off(1)*/));
        }
        Search::Explore => {
            let media = media::list_explore(&state.pool, page).await?;
            map.insert("media".to_string(), json!(media));
        }
        Search::Reply(media_id) => {
            let media = media::list_replies(&state.pool, media_id, page).await?;
            let comments = comment::list(&state.pool, media_id, page).await?;
            // todo: how does page differentiate between media replies page and comments page?
            map.insert("media".to_string(), json!(media));
            map.insert("comments".to_string(), json!(comments));
        }
        _ => {}
    }
    map.retain(|_, v| v.as_array().map_or(true, |a| !a.is_empty()));
    Ok(Json(Value::Object(map)))
}

pub async fn find_by_id(
    state: &AppState,
    media_id: Uuid,
) -> Result<Json<Value>> {
    let media = media::find_by_id(&state.pool, media_id).await?;
    Ok(Json(json!({ "media": media })))
}

pub async fn create(
    state: &AppState,
    user_id: Uuid,
    parent_media_id: Option<Uuid>,
    r#type: MediaType,
    caption: Option<String>,
) -> Result<Uuid> {
    let media_id = media::create(&state.pool, user_id, parent_media_id, r#type, caption).await?;
    Ok(media_id)
}

pub async fn activate(
    state: &AppState,
    user_id: Uuid,
    media_id: Uuid,
    embedding: Vector,
    blurhash: Option<String>,
) -> Result<Media> {
    let result = media::activate(&state.pool, user_id, media_id, embedding, blurhash).await?;
    Ok(result)
}

pub async fn update(
    state: &AppState,
    user_id: Uuid,
    media_id: Uuid,
    caption: Option<String>,
) -> Result<Media> {
    // Keep the current embedding, so I don't have to download the AVIF and convert it to JPEG and resize again
    let result = media::update(&state.pool, user_id, media_id, caption).await?;
    Ok(result)
}

pub async fn remove(
    state: &AppState,
    user_id: Uuid,
    media_id: Uuid,
) -> Result<()> {
    media::remove(&state.pool, user_id, media_id).await?;
    Ok(())
}