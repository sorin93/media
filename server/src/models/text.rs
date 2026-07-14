use pgvector::Vector;
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Serialize)]
pub struct Text {
    pub text_id: Uuid,
    pub embedding: Vector,
}