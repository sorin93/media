use sqlx::{PgPool, query, query_as};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::error::Result;
use crate::comment::model::Comment;

pub async fn create(
    pool: &PgPool,
    user_id: Uuid,
    media_id: Uuid,
    content: &str,
) -> Result<Comment> {
    let comment_id = Uuid::new_v4();
    let result = query_as::<_, Comment>(
        r#"
        INSERT INTO comment (comment_id, user_id, media_id, content)
        VALUES ($1, $2, $3, $4)
        RETURNING comment_id, media_id, user_id, content, created_at
        "#,
    )
        .bind(comment_id)
        .bind(user_id)
        .bind(media_id)
        .bind(content)
        .fetch_one(pool)
        .await?;
    Ok(result)
}

pub async fn list_by_media(
    pool: &PgPool,
    media_id: Uuid,
    page: i16,
) -> Result<Vec<Comment>> {
    let offset = (page - 1) * 50;
    let result = query_as::<_, Comment>(
        r#"
        SELECT comment_id, media_id, user_id, content, created_at
        FROM comment
        WHERE media_id = $1
        ORDER BY created_at DESC
        OFFSET $2 LIMIT 50
        "#,
    )
        .bind(media_id)
        .bind(offset)
        .fetch_all(pool)
        .await?;
    Ok(result)
}

pub async fn remove(
    pool: &PgPool,
    comment_id: Uuid,
    user_id: Uuid,
) -> Result<()> {
    query(
        r#"
        DELETE FROM comment
        WHERE comment_id = $1 AND user_id = $2
        "#,
    )
        .bind(comment_id)
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}
