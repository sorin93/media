use pgvector::Vector;
use sqlx::{PgPool, query, query_as};
use uuid::Uuid;

use crate::error::Result;
use crate::models::media::{Media, MediaRow, MediaType};

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
        "#
    )
        .bind(media_id)
        .fetch_one(pool)
        .await?;
    Ok(media.into())
}

pub async fn user_media(pool: &PgPool, user_id: Uuid, page: i16) -> Result<Vec<Media>> {
    let media = query_as::<_, MediaRow>(
        r#"
        SELECT
            u.user_id, u.media_id as user_media_id, u.name as user_name,
            m.media_id, m.parent_media_id, m.type, m.caption, m.created_at, m.reply_count, m.comment_count, m.like_count, m.embedding, m.blurhash
        FROM media m
        INNER JOIN "user" u ON u.user_id = m.user_id
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
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    Ok(media)
}

pub async fn user_replies(pool: &PgPool, user_id: Uuid, page: i16) -> Result<Vec<Media>> {
    let media = query_as::<_, MediaRow>(
        r#"
        SELECT
            u.user_id, u.media_id as user_media_id, u.name as user_name,
            m.media_id, m.parent_media_id, m.type, m.caption, m.created_at, m.reply_count, m.comment_count, m.like_count, m.embedding, m.blurhash
        FROM media m
        INNER JOIN "user" u ON u.user_id = m.user_id
        WHERE m.user_id = $1
          AND m.parent_media_id IS NOT NULL
          AND m.status = 'active'
          AND u.status = 'active'
        ORDER BY m.created_at DESC
        OFFSET $2
        LIMIT 100
        "#
    )
        .bind(user_id)
        .bind((page - 1) * 100)
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    Ok(media)
}

pub async fn media_replies(pool: &PgPool, media_id: Uuid, page: i16) -> Result<Vec<Media>> {
    let media = query_as::<_, MediaRow>(
        r#"
        SELECT
            u.user_id, u.media_id as user_media_id, u.name as user_name,
            m.media_id, m.parent_media_id, m.type, m.caption, m.created_at, m.reply_count, m.comment_count, m.like_count, m.embedding, m.blurhash
        FROM media m
        INNER JOIN "user" u ON u.user_id = m.user_id
        WHERE m.parent_media_id = $1
          AND m.status = 'active'
          AND u.status = 'active'
        ORDER BY m.created_at DESC
        OFFSET $2
        LIMIT 100
        "#
    )
        .bind(media_id)
        .bind((page - 1) * 100)
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    Ok(media)
}

pub async fn list_user(pool: &PgPool, user_id: Uuid, page: i16) -> Result<Vec<Media>> {
    let media = query_as::<_, MediaRow>(
        r#"
        SELECT
            u.user_id, u.media_id as user_media_id, u.name as user_name,
            m.media_id, m.parent_media_id, m.type, m.caption, m.created_at, m.reply_count, m.comment_count, m.like_count, m.embedding, m.blurhash
        FROM media m
        INNER JOIN "user" u ON u.user_id = m.user_id
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
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    Ok(media)
}

pub async fn list_liked(pool: &PgPool, user_id: Uuid, page: i16) -> Result<Vec<Media>> {
    let media = query_as::<_, MediaRow>(
        r#"
        SELECT
            u.user_id, u.media_id as user_media_id, u.name as user_name,
            m.media_id, m.parent_media_id, m.type, m.caption, m.created_at, m.reply_count, m.comment_count, m.like_count, m.embedding, m.blurhash
        FROM media m
        INNER JOIN "user" u ON u.user_id = m.user_id
        INNER JOIN "like" l ON l.media_id = m.media_id
        WHERE l.user_id = $1
          AND u.status = 'active'
          AND m.status = 'active'
        ORDER BY l.created_at DESC
        OFFSET $2
        LIMIT 100
        "#
    )
        .bind(user_id)
        .bind((page - 1) * 100)
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    Ok(media)
}

pub async fn list_replied(pool: &PgPool, user_id: Uuid, page: i16) -> Result<Vec<Media>> {
    let media = query_as::<_, MediaRow>(
        r#"
        SELECT
            u.user_id, u.media_id as user_media_id, u.name as user_name,
            m.media_id, m.parent_media_id, m.type, m.caption, m.created_at, m.reply_count, m.comment_count, m.like_count, m.embedding, m.blurhash
        FROM media m
        INNER JOIN "user" u ON u.user_id = m.user_id
        WHERE m.user_id = $1
          AND m.parent_media_id IS NOT NULL
          AND m.status = 'active'
          AND u.status = 'active'
        ORDER BY m.created_at DESC
        OFFSET $2
        LIMIT 100
        "#
    )
        .bind(user_id)
        .bind((page - 1) * 100)
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    Ok(media)
}

pub async fn list_following(pool: &PgPool, user_id: Uuid, page: i16) -> Result<Vec<Media>> {
    let media = query_as::<_, MediaRow>(
        r#"
        SELECT
            u.user_id, u.media_id as user_media_id, u.name as user_name,
            m.media_id, m.parent_media_id, m.type, m.caption, m.created_at, m.reply_count, m.comment_count, m.like_count, m.embedding, m.blurhash
        FROM media m
        INNER JOIN "user" u ON u.user_id = m.user_id
        INNER JOIN follow f ON f.to_user_id = m.user_id
        WHERE f.from_user_id = $1
          AND m.status = 'active'
          AND u.status = 'active'
        ORDER BY m.created_at DESC
        OFFSET $2
        LIMIT 100
        "#
    )
        .bind(user_id)
        .bind((page - 1) * 100)
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    Ok(media)
}

pub async fn list_similar_by_id(pool: &PgPool, media_id: Uuid, page: i16) -> Result<Vec<Media>> {
    let limit = 100 + (page == 1) as i16;
    let media = query_as::<_, MediaRow>(
        r#"
        SELECT
            u.user_id, u.media_id as user_media_id, u.name as user_name,
            m.media_id, m.parent_media_id, m.type, m.caption, m.created_at, m.reply_count, m.comment_count, m.like_count, m.embedding, m.blurhash
        FROM media m
        INNER JOIN "user" u ON u.user_id = m.user_id
        WHERE m.status = 'active'
          AND u.status = 'active'
        ORDER BY
            subvector(m.embedding, 1, 768)::vector(768)
            <=>
            (SELECT subvector(embedding, 1, 768)::vector(768) FROM media WHERE media_id = $1)
        OFFSET $2
        LIMIT $3
        "#
    )
        .bind(media_id)
        .bind((page - 1) * 100 + 1) // skip first entry
        .bind(limit)
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    Ok(media)
}

pub async fn list_similar_by_embedding(
    pool: &PgPool,
    embedding: Vector,
    page: i16,
) -> Result<Vec<Media>> {
    let limit = 100 + (page == 1) as i16;
    let media = query_as::<_, MediaRow>(
        r#"
        SELECT
            u.user_id, u.media_id as user_media_id, u.name as user_name,
            m.media_id, m.parent_media_id, m.type, m.caption, m.created_at, m.reply_count, m.comment_count, m.like_count, m.embedding, m.blurhash
        FROM media m
        INNER JOIN "user" u ON u.user_id = m.user_id
        WHERE m.status = 'active'
          AND u.status = 'active'
        ORDER BY
            subvector(m.embedding, 1, 768)::vector(768)
            <=>
            subvector($1::vector, 1, 768)::vector(768)
        OFFSET $2
        LIMIT $3
        "#
    )
        .bind(embedding)
        .bind((page - 1) * 100)
        .bind(limit)
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    Ok(media)
}

pub async fn list_replies(pool: &PgPool, media_id: Uuid, page: i16) -> Result<Vec<Media>> {
    let media = query_as::<_, MediaRow>(
        r#"
        SELECT
            u.user_id, u.media_id as user_media_id, u.name as user_name,
            m.media_id, m.parent_media_id, m.type, m.caption, m.created_at, m.reply_count, m.comment_count, m.like_count, m.embedding, m.blurhash
        FROM media m
        INNER JOIN "user" u ON u.user_id = m.user_id
        WHERE m.parent_media_id = $1
          AND m.status = 'active'
          AND u.status = 'active'
        ORDER BY m.created_at DESC
        OFFSET $2
        LIMIT 100
        "#
    )
        .bind(media_id)
        .bind((page - 1) * 100)
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    Ok(media)
}

pub async fn list_explore(pool: &PgPool, page: i16) -> Result<Vec<Media>> {
    let media = query_as::<_, MediaRow>(
        r#"
        SELECT
            u.user_id, u.media_id as user_media_id, u.name as user_name,
            m.media_id, m.parent_media_id, m.type, m.caption, m.created_at, m.reply_count, m.comment_count, m.like_count, m.embedding, m.blurhash
        FROM media m
        INNER JOIN "user" u ON u.user_id = m.user_id
          AND m.status = 'active'
          AND u.status = 'active'
        ORDER BY RANDOM()
        OFFSET $1
        LIMIT 100
        "#
    )
        .bind((page - 1) * 100)
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    Ok(media)
}

/*
pub async fn list(
    pool: &PgPool,
    user_id: Option<Uuid>,
    search: Search,
    embedding: Option<Vector>,
    page: i16,
) -> Result<Vec<MediaRow>> {
// ) -> Result<Vec<Media>> {
    let media = match search {
        Search::Explore => {
            query_as::<_, MediaRow>(
                r#"
                SELECT
                    u.user_id, u.media_id as user_media_id, u.name as user_name,
                    m.media_id, m.parent_media_id, m.type, m.caption, m.created_at, m.reply_count, m.comment_count, m.like_count, m.embedding, m.blurhash
                FROM media m
                INNER JOIN "user" u ON u.user_id = m.user_id
              AND m.status = 'active'
              AND u.status = 'active'
                ORDER BY RANDOM()
                LIMIT 100
                "#
            )
        }
        Search::Following => {
            let user_id = user_id.ok_or(Error::Unauthorized("Unauthorized".into()))?;
            query_as::<_, MediaRow>(
                r#"
                SELECT
                    u.user_id, u.media_id as user_media_id, u.name as user_name,
                    m.media_id, m.parent_media_id, m.type, m.caption, m.created_at, m.reply_count, m.comment_count, m.like_count, m.embedding, m.blurhash
                FROM media m
                INNER JOIN "user" u ON u.user_id = m.user_id
                INNER JOIN follow f ON f.to_user_id = m.user_id
                WHERE f.from_user_id = $1
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
                    m.media_id, m.parent_media_id, m.type, m.caption, m.created_at, m.reply_count, m.comment_count, m.like_count, m.embedding, m.blurhash
                FROM media m
                INNER JOIN "user" u ON u.user_id = m.user_id
                WHERE m.media_id = $1
              AND m.status = 'active'
              AND u.status = 'active'
                "#
            )
                .bind(media_id)
        }
        Search::Similar(ref media_ids) => {
            if !(1..=4).contains(&media_ids.len()) {
                return Err(Error::UnprocessableEntity("No media target".into()));
            }
            query_as::<_, MediaRow>(
                r#"
                SELECT
                    u.user_id, u.media_id as user_media_id, u.name as user_name,
                    m.media_id, m.parent_media_id, m.type, m.caption, m.created_at, m.reply_count, m.comment_count, m.like_count, m.embedding, m.blurhash
                FROM media m
                INNER JOIN "user" u ON u.user_id = m.user_id
                WHERE m.status = 'active'
              AND u.status = 'active'
                ORDER BY (subvector(m.embedding, 1, 768)::vector(768)) <=> (
                    SELECT subvector(avg(embedding)::vector, 1, 768)::vector(768)
                    FROM media
                    WHERE media_id = ANY($1)
                )
                OFFSET $2
                LIMIT 100
                "#
            )
                .bind(media_ids)
                .bind((page - 1) * 100)
        }
        Search::Reply(media_id) => {
            query_as::<_, MediaRow>(
                r#"
                SELECT
                    u.user_id, u.media_id as user_media_id, u.name as user_name,
                    m.media_id, m.parent_media_id, m.type, m.caption, m.created_at, m.reply_count, m.comment_count, m.like_count, m.embedding, m.blurhash
                FROM media m
                INNER JOIN "user" u ON u.user_id = m.user_id
                WHERE m.parent_media_id = $1
              AND m.status = 'active'
              AND u.status = 'active'
                ORDER BY m.created_at DESC
                OFFSET $2
                LIMIT 100
                "#
            )
                .bind(media_id)
                .bind((page - 1) * 100)
        }
        Search::Text(_) => {
            query_as::<_, MediaRow>(
                r#"
                SELECT
                    u.user_id, u.media_id as user_media_id, u.name as user_name,
                    m.media_id, m.parent_media_id, m.type, m.caption, m.created_at, m.reply_count, m.comment_count, m.like_count, m.embedding, m.blurhash
                FROM media m
                INNER JOIN "user" u ON u.user_id = m.user_id
                WHERE m.status = 'active'
              AND u.status = 'active'
                ORDER BY (subvector(m.embedding, 1, 768)::vector(768)) <=> (subvector($1::vector(3072), 1, 768)::vector(768))
                OFFSET $2
                LIMIT 100
                "#
            )
                .bind(embedding)
                .bind((page - 1) * 100)
        }
        Search::User(user_id) => {
            query_as::<_, MediaRow>(
                r#"
                SELECT
                    u.user_id, u.media_id as user_media_id, u.name as user_name,
                    m.media_id, m.parent_media_id, m.type, m.caption, m.created_at, m.reply_count, m.comment_count, m.like_count, m.embedding, m.blurhash
                FROM media m
                INNER JOIN "user" u ON u.user_id = m.user_id
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
        .await?;
        //.into_iter()
        //.map(Into::into)
        //.collect();
    Ok(media)
}
*/

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
    user_id: Uuid,
    media_id: Uuid,
    embedding: Vector,
    blurhash: Option<String>,
) -> Result<Media> {
    let result = query_as::<_, MediaRow>(
        r#"
        WITH updated_media AS (
            UPDATE media SET status = 'active', updated_at = now(), embedding = $3, blurhash = $4
            WHERE user_id = $1 AND media_id = $2 AND status = 'created' AND created_at > now() - INTERVAL '15 minutes'
            RETURNING user_id, media_id, parent_media_id, type, caption, created_at, reply_count, comment_count, embedding, blurhash
        ),
        updated_user AS (
            UPDATE "user"
            SET media_count = media_count + 1, media_id = $2
            WHERE user_id = $1 and status = 'active'
            RETURNING user_id, media_id, name
        )
        SELECT
            u.user_id, u.media_id as user_media_id, u.name as user_name,
            m.media_id, m.parent_media_id, m.type, m.caption, m.created_at, m.reply_count, m.comment_count, m.like_count, m.embedding, m.blurhash
        FROM updated_media m
        INNER JOIN updated_user u ON u.user_id = m.user_id
        "#,
    )
        .bind(user_id)
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
            m.media_id, m.parent_media_id, m.type, m.caption, m.created_at, m.reply_count, m.comment_count, m.like_count, m.embedding, m.blurhash
        FROM updated m
        INNER JOIN "user" u ON u.user_id = m.user_id
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

pub async fn remove(pool: &PgPool, user_id: Uuid, media_id: Uuid) -> Result<()> {
    query!(
        r#"
        WITH updated_media AS (
            UPDATE media
            SET status = 'deleted', updated_at = now(), embedding = NULL, blurhash = NULL
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
