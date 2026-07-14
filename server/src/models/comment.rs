use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::models::user::User;

#[derive(Serialize)]
pub struct Comment {
    pub user: User,
    pub media_id: Uuid,
    pub comment_id: Uuid,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    pub text: String,
}

#[derive(FromRow)]
pub struct CommentRow {
    pub media_id: Uuid,
    pub comment_id: Uuid,
    pub created_at: OffsetDateTime,
    pub text: String,
    pub user_id: Uuid,
    pub user_media_id: Option<Uuid>,
    pub user_name: String,
}

impl From<CommentRow> for Comment {
    fn from(row: CommentRow) -> Self {
        Comment {
            user: User {
                user_id: row.user_id,
                media_id: row.user_media_id,
                name: row.user_name,
            },
            media_id: row.media_id,
            comment_id: row.comment_id,
            created_at: row.created_at,
            text: row.text,
        }
    }
}

#[derive(Deserialize)]
pub struct CreateComment {
    pub text: String,
}