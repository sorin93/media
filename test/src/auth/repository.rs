use sqlx::{PgPool, query, query_as};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::auth::model::{RefreshToken, RefreshTokenStatus};
use crate::error::Result;

// Refresh Token Operations

pub async fn insert_refresh_token(
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

pub async fn find_refresh_token_by_hash(
    pool: &PgPool,
    token_hash: &str,
) -> Result<Option<RefreshToken>> {
    let result = query_as(r#"SELECT user_id, session_id, status, expires_at FROM refresh_token WHERE token_hash = $1"#)
        .bind(token_hash)
        .fetch_optional(pool)
        .await?;
    Ok(result)
}

pub async fn mark_token_used(pool: &PgPool, token_hash: &str) -> Result<()> {
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

// Verification Code Operations

pub async fn create_verification(
    pool: &PgPool,
    email: &str,
    verify_type: crate::auth::model::VerifyType,
    code_hash: &str,
    password_hash: Option<&str>,
    expires_at: OffsetDateTime,
) -> Result<crate::auth::model::Verify> {
    let result = query_as(
        r#"
        INSERT INTO verify (user_id, "type", created_at, expires_at, email, code_hash, password_hash, attempts)
        VALUES (NULL, $1, $2, $3, $4, $5, $6, 0)
        ON CONFLICT (email, type) DO UPDATE SET
            code_hash = EXCLUDED.code_hash,
            password_hash = EXCLUDED.password_hash,
            expires_at = EXCLUDED.expires_at,
            created_at = EXCLUDED.created_at,
            attempts = 0
        RETURNING *
        "#,
    )
        .bind(verify_type)
        .bind(OffsetDateTime::now_utc())
        .bind(expires_at)
        .bind(email)
        .bind(code_hash)
        .bind(password_hash)
        .fetch_one(pool)
        .await?;
    Ok(result)
}

pub async fn find_verification(
    pool: &PgPool,
    email: &str,
    verify_type: crate::auth::model::VerifyType,
) -> Result<Option<crate::auth::model::Verify>> {
    let result = query_as(
        r#"
        SELECT * FROM verify
        WHERE email = $1 AND "type" = $2
        "#,
    )
    .bind(email)
    .bind(verify_type)
    .fetch_optional(pool)
    .await?;
    Ok(result)
}

pub async fn remove_verification(
    pool: &PgPool,
    email: &str,
    verify_type: crate::auth::model::VerifyType,
) -> Result<()> {
    query(
        r#"
        DELETE FROM verify
        WHERE email = $1 AND "type" = $2
        "#,
    )
    .bind(email)
    .bind(verify_type)
    .execute(pool)
    .await?;
    Ok(())
}
