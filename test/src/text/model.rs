use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, FromRow, Serialize)]
pub struct Text {
    pub text_id: Uuid,
    pub text: String,
    pub embedding: pgvector::Vector,
}
