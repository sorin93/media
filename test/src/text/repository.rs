use pgvector::Vector;
use sqlx::{PgPool, query, query_as};
use uuid::Uuid;

use crate::error::Result;
use crate::text::model::Text;

pub async fn find_by_text(pool: &PgPool, text: &str) -> Result<Option<Text>> {
    let result = query_as(
        r#"
        SELECT text_id, text, embedding
        FROM text
        WHERE text = $1
        "#,
    )
    .bind(text)
    .fetch_optional(pool)
    .await?;
    Ok(result)
}

pub async fn find_by_id(pool: &PgPool, text_id: Uuid) -> Result<Option<Text>> {
    let result = query_as(
        r#"
        SELECT text_id, text, embedding
        FROM text
        WHERE text_id = $1
        "#,
    )
    .bind(text_id)
    .fetch_optional(pool)
    .await?;
    Ok(result)
}

pub async fn list_by_embedding(pool: &PgPool, embedding: Vector, page: i16) -> Result<Vec<Text>> {
    let offset = (page - 1) * 50;
    let result = query_as(
        r#"
        SELECT text_id, text, embedding
        FROM text
        ORDER BY embedding <=> $1
        OFFSET $2 LIMIT 50
        "#,
    )
    .bind(embedding)
    .bind(offset)
    .fetch_all(pool)
    .await?;
    Ok(result)
}

pub async fn insert(pool: &PgPool, text: String, embedding: Vector) -> Result<Text> {
    let text_id = Uuid::new_v4();
    let result = query_as(
        r#"
        INSERT INTO text (text_id, text, embedding)
        VALUES ($1, $2, $3)
        RETURNING text_id, text, embedding
        "#,
    )
    .bind(text_id)
    .bind(text)
    .bind(embedding)
    .fetch_one(pool)
    .await?;
    Ok(result)
}
