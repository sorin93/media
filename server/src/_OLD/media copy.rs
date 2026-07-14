use pgvector::Vector;
use sqlx::{PgPool, query, query_as};
use uuid::Uuid;

use crate::error::Result;
use crate::models::media::{Media, MediaType, MediaRow, Search};

pub async fn list(
    pool: &PgPool,
    user_id: Uuid,
    mut search: Search,
    page: i16,
) -> Result<Vec<Media>> {
    let media = match search {
        Search::Explore => {
            query_as::<_, MediaRow>(
                r#"
                SELECT
                    u.user_id, u.media_id as user_media_id, u.name as user_name,
                    m.media_id, m.parent_media_id, m.type, m.caption, m.created_at, m.comment_count, m.reply_count, m.blurhash
                FROM media m
                JOIN "user" u ON u.user_id = m.user_id
                WHERE m.user_id = $1
                AND m.status = 'active'
                AND u.status = 'active'
                ORDER BY m.created_at DESC
                OFFSET $2
                LIMIT 100
                "#
            )
                .bind(user_id)
                .bind((page - 1) * 100)
        }
        Search::Following => {
            query_as::<_, MediaRow>(
                r#"
                SELECT
                    u.user_id, u.media_id as user_media_id, u.name as user_name,
                    m.media_id, m.parent_media_id, m.type, m.caption, m.created_at, m.comment_count, m.reply_count, m.blurhash
                FROM media m
                JOIN "user" u ON u.user_id = m.user_id
                WHERE m.user_id = $1
                AND m.status = 'active'
                AND u.status = 'active'
                ORDER BY m.created_at DESC
                OFFSET $2
                LIMIT 100
                "#
            )
                .bind(user_id)
                .bind((page - 1) * 100)
        }
        Search::Media(media_id) => {
            query_as::<_, MediaRow>(
                r#"
                SELECT
                    u.user_id, u.media_id as user_media_id, u.name as user_name,
                    m.media_id, m.parent_media_id, m.type, m.caption, m.created_at, m.comment_count, m.reply_count, m.blurhash
                FROM media m
                JOIN "user" u ON u.user_id = m.user_id
                WHERE m.media_id = $1
                AND m.status = 'active'
                AND u.status = 'active'
                "#
            )
                .bind(media_id)
        }
        Search::Similar(ref _media_ids) => {
            // todo
            query_as::<_, MediaRow>(
                r#"
                SELECT
                    u.user_id, u.media_id as user_media_id, u.name as user_name,
                    m.media_id, m.parent_media_id, m.type, m.caption, m.created_at, m.comment_count, m.reply_count, m.blurhash
                FROM media m
                JOIN "user" u ON u.user_id = m.user_id
                WHERE m.user_id = $1
                AND m.status = 'active'
                AND u.status = 'active'
                ORDER BY m.created_at DESC
                OFFSET $2
                LIMIT 100
                "#
            )
                .bind(user_id)
                .bind((page - 1) * 100)
        }
        Search::Reply(_media_id) => {
            // todo
            query_as::<_, MediaRow>(
                r#"
                SELECT
                    u.user_id, u.media_id as user_media_id, u.name as user_name,
                    m.media_id, m.parent_media_id, m.type, m.caption, m.created_at, m.comment_count, m.reply_count, m.blurhash
                FROM media m
                JOIN "user" u ON u.user_id = m.user_id
                WHERE m.user_id = $1
                AND m.status = 'active'
                AND u.status = 'active'
                ORDER BY m.created_at DESC
                OFFSET $2
                LIMIT 100
                "#
            )
                .bind(user_id)
                .bind((page - 1) * 100)
        }
        Search::Text(ref mut _text) => {
            // todo
            query_as::<_, MediaRow>(
                r#"
                SELECT
                    u.user_id, u.media_id as user_media_id, u.name as user_name,
                    m.media_id, m.parent_media_id, m.type, m.caption, m.created_at, m.comment_count, m.reply_count, m.blurhash
                FROM media m
                JOIN "user" u ON u.user_id = m.user_id
                WHERE m.user_id = $1
                AND m.status = 'active'
                AND u.status = 'active'
                ORDER BY m.created_at DESC
                OFFSET $2
                LIMIT 100
                "#
            )
                .bind(user_id)
                .bind((page - 1) * 100)
        }
        Search::User(user_id) => {
            query_as::<_, MediaRow>(
                r#"
                SELECT
                    u.user_id, u.media_id as user_media_id, u.name as user_name,
                    m.media_id, m.parent_media_id, m.type, m.caption, m.created_at, m.comment_count, m.reply_count, m.blurhash
                FROM media m
                JOIN "user" u ON u.user_id = m.user_id
                WHERE m.user_id = $1
                AND m.status = 'active'
                AND u.status = 'active'
                ORDER BY m.created_at DESC
                OFFSET $2
                LIMIT 100
                "#
            )
                .bind(user_id)
                .bind((page - 1) * 100)
        }
    }
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();

    Ok(media)
}

pub async fn create(
    pool: &PgPool,
    user_id: Uuid,
    parent_media_id: Option<Uuid>,
    r#type: MediaType,
    caption: Option<String>,
) -> Result<Uuid> {
    let media_id = sqlx::query_scalar::<_, Uuid>(
        r#"
        INSERT INTO media (user_id, parent_media_id, type, caption)
        VALUES ($1, $2, $3, $4)
        RETURNING media_id
        "#,
    )
        .bind(user_id)
        .bind(parent_media_id)
        .bind(r#type)
        .bind(caption)
        .fetch_one(pool)
        .await?;
    Ok(media_id)
}

pub async fn activate(
    pool: &PgPool,
    media_id: Uuid,
    embedding: Vector,
    blurhash: String,
) -> Result<Media> {
    let result = query_as::<_, MediaRow>(
        r#"
        WITH updated_media AS (
            UPDATE media SET status = 'active', updated_at = now(), embedding = $2, blurhash = $3
            WHERE media_id = $1 AND status = 'created'
            RETURNING user_id, media_id, parent_media_id, type, caption, created_at, reply_count, comment_count, blurhash
        ),
        updated_user AS (
            UPDATE "user"
            SET media_count = media_count + 1, media_id = $1
            WHERE user_id = (SELECT user_id FROM updated_media) and status = 'active'
            RETURNING user_id, media_id, name
        )
        SELECT
            u.user_id, u.media_id as user_media_id, u.name as user_name,
            m.media_id, m.parent_media_id, m.type, m.caption, m.created_at, m.comment_count, m.reply_count, m.blurhash
        FROM updated_media m
        JOIN updated_user u ON u.user_id = m.user_id
        "#,
    )
        .bind(media_id)
        .bind(embedding)
        .bind(blurhash)
        .fetch_one(pool)
        .await?
        .into();
    Ok(result)
}

pub async fn update(
    pool: &PgPool,
    user_id: Uuid,
    media_id: Uuid,
    caption: Option<String>,
) -> Result<Media> {
    let result = query_as::<_, MediaRow>(
        r#"
        WITH updated AS (
            UPDATE media
            SET caption = $3, updated_at = now()
            WHERE user_id = $1 AND media_id = $2 AND status = 'active'
            RETURNING user_id, media_id, parent_media_id, type, caption, created_at, comment_count, reply_count, blurhash
        )
        SELECT
            u.user_id, u.media_id as user_media_id, u.name as user_name,
            m.media_id, m.parent_media_id, m.type, m.caption, m.created_at, m.comment_count, m.reply_count, m.blurhash
        FROM updated m
        JOIN "user" u ON u.user_id = m.user_id
        WHERE u.status = 'active'
        "#,
    )
        .bind(user_id)
        .bind(media_id)
        .bind(caption)
        .fetch_one(pool)
        .await?
        .into();
    Ok(result)
}

pub async fn remove(
    pool: &PgPool,
    user_id: Uuid,
    media_id: Uuid,
) -> Result<()> {
    // Not clearing embedding and blurhash
    query!(
        r#"
        WITH updated_media AS (
            UPDATE media
            SET status = 'deleted', updated_at = now()
            WHERE user_id = $1 AND media_id = $2 AND status = 'active'
            RETURNING 1
        ),
        updated_user AS (
            UPDATE "user"
            SET 
                updated_at = now(),
                media_count = GREATEST(media_count - 1, 0),
                media_id = CASE WHEN media_id = $2 THEN NULL ELSE media_id END
            WHERE user_id = $1 
              AND status = 'active'
              AND EXISTS (SELECT 1 FROM updated_media)
            RETURNING 1
        )
        SELECT 1 as id FROM updated_media
        "#,
        user_id,
        media_id
    )
        .fetch_one(pool)
        .await?;
    Ok(())
}