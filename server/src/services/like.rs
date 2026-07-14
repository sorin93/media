use axum::Json;
use serde_json::{Map, Value, json};
use uuid::Uuid;

use crate::db::{like, media, user};
use crate::error::Result;
use crate::state::AppState;

// Like user
pub async fn user_likes(
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
    let media = like::user_likes(&state.pool, user_id, page).await?;
    map.insert("media".to_string(), json!(media));
    map.retain(|_, v| v.as_array().map_or(true, |a| !a.is_empty()));
    Ok(Json(Value::Object(map)))
}

// Like media
pub async fn media_likes(
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
    let media = like::media_likes(&state.pool, media_id, page).await?;
    map.insert("media".to_string(), json!(media));
    map.retain(|_, v| v.as_array().map_or(true, |a| !a.is_empty()));
    Ok(Json(Value::Object(map)))
}
