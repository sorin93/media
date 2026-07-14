use sqlx::{PgPool, query, query_as};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::error::Result;
use crate::models::auth::{RefreshToken, RefreshTokenStatus};

pub async fn insert(
    pool: &PgPool,
    token_hash: &str,
    user_id: Uuid,
    session_id: Uuid,
    expires_at: OffsetDateTime,
) -> Result<RefreshToken> {
    let result = query_as(
        r#"
        INSERT INTO refresh_token (token_hash, user_id, session_id, status, created_at, expires_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING user_id, session_id, status, expires_at
        "#,
    )
        .bind(token_hash)
        .bind(user_id)
        .bind(session_id)
        .bind(RefreshTokenStatus::Active)
        .bind(OffsetDateTime::now_utc())
        .bind(expires_at)
        .fetch_one(pool)
        .await?;
    Ok(result)
}

pub async fn find_by_hash(pool: &PgPool, token_hash: &str) -> Result<Option<RefreshToken>> {
    let result = query_as(r#"SELECT user_id, session_id, status, expires_at FROM refresh_token WHERE token_hash = $1"#)
        .bind(token_hash)
        .fetch_optional(pool)
        .await?;
    Ok(result)
}

pub async fn mark_used(pool: &PgPool, token_hash: &str) -> Result<()> {
    query(
        r#"
        UPDATE refresh_token
        SET status = $2
        WHERE token_hash = $1
        "#,
    )
        .bind(token_hash)
        .bind(RefreshTokenStatus::Used)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn revoke_session(pool: &PgPool, session_id: Uuid) -> Result<u64> {
    let count = query(
        r#"
        UPDATE refresh_token
        SET status = $2
        WHERE session_id = $1 AND status != $2
        "#,
    )
        .bind(session_id)
        .bind(RefreshTokenStatus::Revoked)
        .execute(pool)
        .await?
        .rows_affected();
    Ok(count)
}

pub async fn revoke_all_for_user(pool: &PgPool, user_id: Uuid) -> Result<u64> {
    let count = query(
        r#"
        UPDATE refresh_token
        SET status = $2
        WHERE user_id = $1 AND status != $2
        "#,
    )
        .bind(user_id)
        .bind(RefreshTokenStatus::Revoked)
        .execute(pool)
        .await?
        .rows_affected();
    Ok(count)
}

// todo: housecleaning, to be run by a worker or something
#[allow(dead_code)]
pub async fn delete_expired(pool: &PgPool) -> Result<u64> {
    let count = query(r#"DELETE FROM refresh_token WHERE expires_at < $1"#)
        .bind(OffsetDateTime::now_utc())
        .execute(pool)
        .await?
        .rows_affected();
    Ok(count)
}