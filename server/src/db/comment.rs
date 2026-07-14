use sqlx::{PgPool, query, query_as};
use uuid::Uuid;

use crate::error::Result;
use crate::models::comment::{Comment, CommentRow};

pub async fn user_comments(pool: &PgPool, media_id: Uuid, page: i16) -> Result<Vec<Comment>> {
    let result = query_as::<_, CommentRow>(
        r#"
        SELECT
            c.media_id,
            c.comment_id,
            c.created_at,
            c.text,
            u.user_id,
            u.media_id AS user_media_id,
            u.name AS user_name
        FROM comment c
        INNER JOIN "user" u ON u.user_id = c.from_user_id
        WHERE c.user_id = $1
          AND c.status = 'active'
          AND u.status = 'active'
        ORDER BY c.created_at DESC
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

pub async fn media_comments(pool: &PgPool, media_id: Uuid, page: i16) -> Result<Vec<Comment>> {
    let result = query_as::<_, CommentRow>(
        r#"
        SELECT
            c.media_id,
            c.comment_id,
            c.created_at,
            c.text,
            u.user_id,
            u.media_id AS user_media_id,
            u.name AS user_name
        FROM comment c
        INNER JOIN "user" u ON u.user_id = c.from_user_id
        WHERE c.media_id = $1
          AND c.status = 'active'
          AND u.status = 'active'
        ORDER BY c.created_at DESC
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

pub async fn list(pool: &PgPool, media_id: Uuid, page: i16) -> Result<Vec<Comment>> {
    let result = query_as::<_, CommentRow>(
        r#"
        SELECT
            c.media_id,
            c.comment_id,
            c.created_at,
            c.text,
            u.user_id,
            u.media_id AS user_media_id,
            u.name AS user_name
        FROM comment c
        INNER JOIN "user" u ON u.user_id = c.from_user_id
        WHERE c.media_id = $1
          AND c.status = 'active'
          AND u.status = 'active'
        ORDER BY c.created_at DESC
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

pub async fn create(pool: &PgPool, user_id: Uuid, media_id: Uuid, text: String) -> Result<Comment> {
    let result = query_as::<_, CommentRow>(
        r#"
        WITH updated_media AS (
            UPDATE media
            SET comment_count = comment_count + 1
            WHERE media_id = $2
              AND status = 'active'
            RETURNING user_id
        ),
        inserted_comment AS (
            INSERT INTO comment (from_user_id, to_user_id, media_id, text)
            SELECT $1, um.user_id, $2, $3
            FROM updated_media um
            ON CONFLICT (media_id, from_user_id) DO NOTHING
            RETURNING comment_id, media_id, created_at, text
        )
        SELECT
            u.user_id, u.media_id AS user_media_id, u.name AS user_name,
            ic.media_id, ic.comment_id, ic.created_at, ic.text
        FROM inserted_comment ic
        INNER JOIN "user" u ON u.user_id = ic.from_user_id
        "#,
    )
    .bind(user_id)
    .bind(media_id)
    .bind(text)
    .fetch_one(pool)
    .await?
    .into();
    Ok(result)
}

pub async fn remove(pool: &PgPool, user_id: Uuid, media_id: Uuid, comment_id: Uuid) -> Result<()> {
    query!(
        r#"
        WITH soft_deleted AS (
            UPDATE comment
            SET status = 'deleted', updated_at = now()
            WHERE comment_id = $3
              AND media_id = $2
              AND status = 'active'
              AND (from_user_id = $1 OR to_user_id = $1)
            RETURNING 1
        ),
        media_updated AS (
            UPDATE media
            SET comment_count = GREATEST(comment_count - 1, 0)
            WHERE media_id = $2
              AND status = 'active'
              AND EXISTS (SELECT 1 FROM soft_deleted)
        )
        SELECT 1 as id FROM soft_deleted
        "#,
        user_id,
        media_id,
        comment_id
    )
    .fetch_one(pool)
    .await?;
    Ok(())
}
