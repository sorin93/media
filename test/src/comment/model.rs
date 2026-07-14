use serde::Deserialize;
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateComment {
    pub content: String,
}

#[derive(Deserialize)]
pub struct CommentParams {
    pub page: Option<i16>,
}

#[derive(Clone, FromRow)]
pub struct Comment {
    pub comment_id: Uuid,
    pub media_id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub created_at: OffsetDateTime,
}
