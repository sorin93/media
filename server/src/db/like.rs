use sqlx::{PgPool, query_as};
use uuid::Uuid;

use crate::error::Result;
use crate::models::like::{Like, LikeRow};

// Get user likes
pub async fn user_likes(pool: &PgPool, user_id: Uuid, page: i16) -> Result<Vec<Like>> {
    let result = query_as::<_, LikeRow>(
        r#"
        SELECT
            l.media_id,
            l.created_at,
            u.user_id,
            u.media_id AS user_media_id,
            u.name AS user_name
        FROM "like" l
        INNER JOIN "user" u ON u.user_id = l.user_id
        WHERE l.user_id = $1
          AND u.status = 'active'
        ORDER BY l.created_at DESC
        OFFSET $2
        LIMIT 100
        "#,
    )
    .bind(user_id)
    .bind((page - 1) * 100)
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(Into::into)
    .collect();
    Ok(result)
}

// Get media likes
pub async fn media_likes(pool: &PgPool, media_id: Uuid, page: i16) -> Result<Vec<Like>> {
    let result = query_as::<_, LikeRow>(
        r#"
        SELECT
            l.media_id,
            l.created_at,
            u.user_id,
            u.media_id AS user_media_id,
            u.name AS user_name
        FROM "like" l
        INNER JOIN "user" u ON u.user_id = l.user_id
        WHERE l.media_id = $1
          AND u.status = 'active'
        ORDER BY l.created_at DESC
        OFFSET $2
        LIMIT 100
        "#,
    )
    .bind(media_id)
    .bind((page - 1) * 100)
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(Into::into)
    .collect();
    Ok(result)
}
