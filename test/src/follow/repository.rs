use sqlx::{PgPool, query, query_as};
use uuid::Uuid;

use crate::error::Result;

pub async fn create(pool: &PgPool, user_id: Uuid, target_user_id: Uuid) -> Result<()> {
    query(
        r#"
        INSERT INTO follow (user_id, target_user_id)
        VALUES ($1, $2)
        ON CONFLICT DO NOTHING
        "#,
    )
    .bind(user_id)
    .bind(target_user_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn remove(pool: &PgPool, user_id: Uuid, target_user_id: Uuid) -> Result<()> {
    query(
        r#"
        DELETE FROM follow
        WHERE user_id = $1 AND target_user_id = $2
        "#,
    )
    .bind(user_id)
    .bind(target_user_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn list_followers(pool: &PgPool, user_id: Uuid, page: i16) -> Result<Vec<Uuid>> {
    let offset = (page - 1) * 50;
    let result = query_as(
        r#"
        SELECT user_id FROM follow
        WHERE target_user_id = $1
        ORDER BY created_at DESC
        OFFSET $2 LIMIT 50
        "#,
    )
    .bind(user_id)
    .bind(offset)
    .fetch_all(pool)
    .await?;
    Ok(result.into_iter().map(|(id,)| id).collect())
}

pub async fn list_following(pool: &PgPool, user_id: Uuid, page: i16) -> Result<Vec<Uuid>> {
    let offset = (page - 1) * 50;
    let result = query_as(
        r#"
        SELECT target_user_id FROM follow
        WHERE user_id = $1
        ORDER BY created_at DESC
        OFFSET $2 LIMIT 50
        "#,
    )
    .bind(user_id)
    .bind(offset)
    .fetch_all(pool)
    .await?;
    Ok(result.into_iter().map(|(id,)| id).collect())
}
