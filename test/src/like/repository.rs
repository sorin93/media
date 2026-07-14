use sqlx::{PgPool, query, query_as};
use uuid::Uuid;

use crate::error::Result;

pub async fn create(pool: &PgPool, user_id: Uuid, media_id: Uuid) -> Result<()> {
    query(
        r#"
        INSERT INTO "like" (user_id, media_id)
        VALUES ($1, $2)
        ON CONFLICT DO NOTHING
        "#,
    )
    .bind(user_id)
    .bind(media_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn remove(pool: &PgPool, user_id: Uuid, media_id: Uuid) -> Result<()> {
    query(
        r#"
        DELETE FROM "like"
        WHERE user_id = $1 AND media_id = $2
        "#,
    )
    .bind(user_id)
    .bind(media_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn list_by_media(pool: &PgPool, media_id: Uuid, page: i16) -> Result<Vec<Uuid>> {
    let offset = (page - 1) * 50;
    let result = query_as(
        r#"
        SELECT user_id FROM "like"
        WHERE media_id = $1
        ORDER BY created_at DESC
        OFFSET $2 LIMIT 50
        "#,
    )
    .bind(media_id)
    .bind(offset)
    .fetch_all(pool)
    .await?;
    Ok(result.into_iter().map(|(id,)| id).collect())
}
