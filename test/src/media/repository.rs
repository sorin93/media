use pgvector::Vector;
use sqlx::{PgPool, query, query_as};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::error::Result;
use crate::media::model::{Media, MediaRow, MediaType};

pub async fn find_by_id(pool: &PgPool, media_id: Uuid) -> Result<Media> {
    let media = query_as::<_, MediaRow>(
        r#"
        SELECT
            u.user_id, u.media_id as user_media_id, u.name as user_name,
            m.media_id, m.parent_media_id, m.type, m.caption, m.created_at, m.reply_count, m.comment_count, m.like_count, m.embedding, m.blurhash
        FROM media m
        INNER JOIN "user" u ON u.user_id = m.user_id
        WHERE m.media_id = $1
          AND m.status = 'active'
          AND u.status = 'active'
        "#,
    )
        .bind(media_id)
        .fetch_one(pool)
        .await?;
    Ok(media.into())
}

pub async fn list_explore(pool: &PgPool, page: i16) -> Result<Vec<Media>> {
    let offset = (page - 1) * 100;
    let media = query_as::<_, MediaRow>(
        r#"
        SELECT
            u.user_id, u.media_id as user_media_id, u.name as user_name,
            m.media_id, m.parent_media_id, m.type, m.caption, m.created_at, m.reply_count, m.comment_count, m.like_count, m.embedding, m.blurhash
        FROM media m
        INNER JOIN "user" u ON u.user_id = m.user_id
        WHERE m.status = 'active'
          AND u.status = 'active'
        ORDER BY m.created_at DESC
        OFFSET $1
        LIMIT 100
        "#,
    )
        .bind(offset)
        .fetch_all(pool)
        .await?;
    Ok(media.into_iter().map(Into::into).collect())
}

pub async fn create(
    pool: &PgPool,
    user_id: Uuid,
    parent_media_id: Option<Uuid>,
    r#type: MediaType,
    caption: Option<String>,
) -> Result<Uuid> {
    let media_id = Uuid::new_v4();
    query(
        r#"
        INSERT INTO media (media_id, user_id, parent_media_id, type, caption, status)
        VALUES ($1, $2, $3, $4, $5, 'created')
        "#,
    )
    .bind(media_id)
    .bind(user_id)
    .bind(parent_media_id)
    .bind(r#type)
    .bind(caption)
    .execute(pool)
    .await?;
    Ok(media_id)
}

pub async fn activate(
    pool: &PgPool,
    user_id: Uuid,
    media_id: Uuid,
    embedding: Vector,
    blurhash: Option<String>,
) -> Result<Media> {
    let media = query_as::<_, MediaRow>(
        r#"
        UPDATE media
        SET status = 'active', embedding = $3, blurhash = $4, updated_at = $5
        WHERE media_id = $1 AND user_id = $2 AND status = 'created'
        RETURNING
            u.user_id, u.media_id as user_media_id, u.name as user_name,
            m.media_id, m.parent_media_id, m.type, m.caption, m.created_at, m.reply_count, m.comment_count, m.like_count, m.embedding, m.blurhash
        FROM "user" u
        WHERE m.user_id = u.user_id
        "#,
    )
        .bind(media_id)
        .bind(user_id)
        .bind(embedding)
        .bind(blurhash)
        .bind(OffsetDateTime::now_utc())
        .fetch_one(pool)
        .await?;
    Ok(media.into())
}

pub async fn update(
    pool: &PgPool,
    user_id: Uuid,
    media_id: Uuid,
    caption: Option<String>,
) -> Result<Media> {
    let media = query_as::<_, MediaRow>(
        r#"
        UPDATE media
        SET caption = $3, updated_at = $4
        WHERE media_id = $1 AND user_id = $2 AND status = 'active'
        RETURNING
            u.user_id, u.media_id as user_media_id, u.name as user_name,
            m.media_id, m.parent_media_id, m.type, m.caption, m.created_at, m.reply_count, m.comment_count, m.like_count, m.embedding, m.blurhash
        FROM "user" u
        WHERE m.user_id = u.user_id
        "#,
    )
        .bind(media_id)
        .bind(user_id)
        .bind(caption)
        .bind(OffsetDateTime::now_utc())
        .fetch_one(pool)
        .await?;
    Ok(media.into())
}

pub async fn remove(pool: &PgPool, user_id: Uuid, media_id: Uuid) -> Result<()> {
    query(
        r#"
        UPDATE media
        SET status = 'deleted', updated_at = $3
        WHERE media_id = $1 AND user_id = $2 AND status = 'active'
        "#,
    )
    .bind(media_id)
    .bind(user_id)
    .bind(OffsetDateTime::now_utc())
    .execute(pool)
    .await?;
    Ok(())
}
