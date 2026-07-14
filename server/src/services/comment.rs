use axum::Json;
use serde_json::{Map, Value, json};
use uuid::Uuid;

use crate::db::{comment, media, user};
use crate::error::Result;
use crate::state::AppState;

pub async fn user_comments(
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
    let comments = comment::user_comments(&state.pool, user_id, page).await?;
    let replies = media::user_replies(&state.pool, user_id, page).await?;
    map.insert("comments".to_string(), json!(comments));
    map.insert("replies".to_string(), json!(replies));
    // todo: combibe both
    map.retain(|_, v| v.as_array().map_or(true, |a| !a.is_empty()));
    Ok(Json(Value::Object(map)))
}

pub async fn media_comments(
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
    let comments = comment::media_comments(&state.pool, media_id, page).await?;
    let replies = media::media_replies(&state.pool, media_id, page).await?;
    map.insert("comments".to_string(), json!(comments));
    map.insert("replies".to_string(), json!(replies));
    // todo: combibe both
    map.retain(|_, v| v.as_array().map_or(true, |a| !a.is_empty()));
    Ok(Json(Value::Object(map)))
}
