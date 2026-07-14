use pgvector::Vector;
use sqlx::{PgPool, query_as, query_scalar};
use uuid::Uuid;

use crate::error::Result;
use crate::models::text::Text;

pub async fn list_by_id(
    pool: &PgPool,
    media_id: Uuid,
) -> Result<Vec<String>> {
    let result = query_scalar(
        r#"
        SELECT text
        FROM "text"
        WHERE status = 'active'
        ORDER BY
            subvector(embedding, 1, 768)::vector(768)
            <=>
            (SELECT subvector(embedding, 1, 768)::vector(768) FROM media WHERE media_id = $1)
        LIMIT 5
        "#
    )
        .bind(media_id)
        .fetch_all(pool)
        .await?;
    Ok(result)
}

pub async fn list_by_embedding(
    pool: &PgPool,
    embedding: Vector,
) -> Result<Vec<String>> {
    let result = query_scalar(
        r#"
        SELECT text
        FROM "text"
        WHERE status = 'active'
        ORDER BY
            subvector(embedding, 1, 768)::vector(768)
            <=>
            subvector($1::vector, 1, 768)::vector(768)
        LIMIT 5
        "#
    )
        .bind(embedding)
        .fetch_all(pool)
        .await?;
    Ok(result)
}

pub async fn find_by_text(
    pool: &PgPool,
    text: &str,
) -> Result<Option<Text>> {
    let result = query_as::<_, Text>(
        r#"
        UPDATE "text"
        SET
            updated_at = now(),
            search_count = search_count + 1
        WHERE "text" = $1
        RETURNING text_id, embedding
        "#
    )
        .bind(text)
        .fetch_optional(pool)
        .await?;
    Ok(result)
}

pub async fn insert(
    pool: &PgPool,
    text: String,
    embedding: Vector,
) -> Result<Text> {
    let result = query_as::<_, Text>(
        r#"
        INSERT INTO "text" ("text", embedding)
        VALUES ($1, $2)
        ON CONFLICT ("text") DO UPDATE
        SET
            updated_at = now(),
            search_count = "text".search_count + 1,
            embedding = EXCLUDED.embedding
        RETURNING text_id, embedding
        "#
    )
        .bind(text)
        .bind(embedding)
        .fetch_one(pool)
        .await?;
    Ok(result)
}