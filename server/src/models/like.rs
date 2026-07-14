use serde::Serialize;
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::models::user::User;

#[derive(Serialize)]
pub struct Like {
    pub user: User,
    pub media_id: Uuid,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
}

#[derive(FromRow)]
pub struct LikeRow {
    pub media_id: Uuid,
    pub created_at: OffsetDateTime,
    pub user_id: Uuid,
    pub user_media_id: Option<Uuid>,
    pub user_name: String,
}

impl From<LikeRow> for Like {
    fn from(row: LikeRow) -> Self {
        Like {
            user: User {
                user_id: row.user_id,
                media_id: row.user_media_id,
                name: row.user_name,
            },
            media_id: row.media_id,
            created_at: row.created_at,
        }
    }
}