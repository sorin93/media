use sqlx::{PgPool, query, query_as};
use uuid::Uuid;

use crate::error::Result;
use crate::models::user::User;

// Get followers
pub async fn list(pool: &PgPool, user_id: Uuid, page: i16) -> Result<Vec<User>> {
    let result = query_as(
        r#"
        SELECT user_id, media_id, name
        FROM follow f
        LEFT JOIN "user" u on u.user_id = f.to_user_id
        WHERE from_user_id = $1 AND status = 'active'
        ORDER BY f.created_at DESC
        OFFSET $2
        LIMIT 100
        "#,
    )
    .bind(user_id)
    .bind((page - 1) * 100)
    .fetch_all(pool)
    .await?;
    Ok(result)
}

// Follow
pub async fn create(pool: &PgPool, from_user_id: Uuid, to_user_id: Uuid) -> Result<User> {
    let result = query_as(
        r#"
        WITH inserted AS (
            INSERT INTO follow (from_user_id, to_user_id)
            VALUES ($1, $2)
            ON CONFLICT (from_user_id, to_user_id) DO NOTHING
            RETURNING 1
        ),
        following_updated AS (
            UPDATE "user"
            SET following_count = following_count + 1
            WHERE user_id = $1
              AND status = 'active'
              AND following_count < 1000
              AND EXISTS (SELECT 1 FROM inserted)
        ),
        followed_updated AS (
            UPDATE "user"
            SET followed_count = followed_count + 1
            WHERE user_id = $2
              AND status = 'active'
              AND followed_count < 1000000
              AND EXISTS (SELECT 1 FROM inserted)
        )
        SELECT u.user_id, u.media_id, u.name
        FROM "user" AS u
        WHERE u.user_id = $2 AND u.status = 'active'
          AND EXISTS (SELECT 1 FROM following_updated, followed_updated)
        "#,
    )
    .bind(from_user_id)
    .bind(to_user_id)
    .fetch_one(pool)
    .await?;
    Ok(result)
}

// Unfollow
pub async fn remove(pool: &PgPool, from_user_id: Uuid, to_user_id: Uuid) -> Result<()> {
    query!(
        r#"
        WITH deleted AS (
            DELETE FROM follow
            WHERE from_user_id = $1
              AND to_user_id = $2
            RETURNING from_user_id
        ),
        following_updated AS (
            UPDATE "user"
            SET following_count = GREATEST(following_count - 1, 0)
            WHERE user_id = $1
              AND status = 'active'
              AND EXISTS (SELECT 1 FROM deleted)
        ),
        followed_updated AS (
            UPDATE "user"
            SET followed_count = GREATEST(followed_count - 1, 0)
            WHERE user_id = $2
              AND status = 'active'
              AND EXISTS (SELECT 1 FROM deleted)
        )
        SELECT 1 as id FROM deleted
        "#,
        from_user_id,
        to_user_id
    )
    .fetch_one(pool)
    .await?;
    Ok(())
}
