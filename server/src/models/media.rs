use pgvector::Vector;
use serde::{Deserialize, Deserializer, Serialize};
use sqlx::{FromRow, Type};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::models::user::User;

// Helper function to deserialize a single value or array into Vec
fn deserialize_uuid_vec<'de, D>(deserializer: D) -> Result<Vec<Uuid>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let uuids: Vec<Uuid> = s
        .split(',')
        .filter_map(|part| {
            let trimmed = part.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(Uuid::parse_str(trimmed))
            }
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(serde::de::Error::custom)?;
    let len = uuids.len();
    if len < 1 {
        return Err(serde::de::Error::custom("at least one UUID is required"));
    }
    if len > 3 {
        return Err(serde::de::Error::custom(format!("expected 1-3 UUIDs, got {}", len)));
    }
    Ok(uuids)
}

#[derive(Serialize, Type)]
#[sqlx(type_name = "media_status", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum MediaStatus {
    Created,
    Active,
    Inactive,
    Deleted,
}

#[derive(Clone, Copy, Deserialize, Serialize, Type, PartialEq)]
#[sqlx(type_name = "media_type", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum MediaType {
    Image,
    Video,
}

#[derive(Serialize)]
pub struct Media {
    pub user: User,
    pub parent_media_id: Option<Uuid>,
    pub media_id: Uuid,
    pub r#type: MediaType,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(default)]
    pub caption: Option<String>,
    pub reply_count: i16,
    pub comment_count: i16,
    pub like_count: i32,
    pub blurhash: Option<String>,
}

#[derive(FromRow, Serialize)]
pub struct MediaRow {
    pub user_id: Uuid,
    pub user_media_id: Option<Uuid>,
    pub user_name: String,
    pub parent_media_id: Option<Uuid>,
    pub media_id: Uuid,
    pub r#type: MediaType,
    pub created_at: OffsetDateTime,
    pub caption: Option<String>,
    pub reply_count: i16,
    pub comment_count: i16,
    pub like_count: i32,
    pub embedding: Vector,
    pub blurhash: Option<String>,
}

impl From<MediaRow> for Media {
    fn from(row: MediaRow) -> Self {
        Media {
            user: User { user_id: row.user_id, media_id: row.user_media_id, name: row.user_name },
            parent_media_id: row.parent_media_id,
            media_id: row.media_id,
            r#type: row.r#type,
            created_at: row.created_at,
            caption: row.caption,
            reply_count: row.reply_count,
            comment_count: row.comment_count,
            like_count: row.like_count,
            blurhash: row.blurhash,
        }
    }
}

#[derive(Clone, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "lowercase")]
pub enum Search {
    Explore, // { "type": "explore" }
    Following, // { "type": "following" }
    Media(Uuid), // { "type": "media", "value": "media_id" }
    Reply(Uuid), // { "type": "reply", "value": "media_id" }
    #[serde(deserialize_with = "deserialize_uuid_vec")]
    Similar(Vec<Uuid>), // { "type": "similar", "value": ["media_id1", "media_id2"] }
    Text(String), // { "type": "text", "value": "...text..." } // todo: rename to Query
    User(Uuid), // { "type": "user", "value": "user_id" }
    Liked(Uuid), // { "type": "liked", "value": "user_id" }
    Replied(Uuid), // { "type": "replied", "value": "user_id" }
}

#[derive(Deserialize)]
pub struct Search2 {
    pub page: Option<i16>,
    pub query: String,
}

#[derive(Deserialize)]
pub struct MediaQuery {
    pub page: Option<i16>,
    #[serde(flatten)]
    pub search: Search,
}

#[derive(Deserialize)]
pub struct CreateMedia {
    pub parent_media_id: Option<Uuid>,
    pub r#type: MediaType,
    #[serde(default)]
    pub caption: Option<String>,
}

#[derive(Deserialize)]
pub struct ActivateMedia {
    pub embedding: Vec<f32>,
    pub blurhash: Option<String>,
}
#[derive(Deserialize)]
pub struct UpdateMedia {
    #[serde(default)]
    pub caption: Option<String>,
}