use sqlx::{PgPool, query, query_as};
use time::OffsetDateTime;

use crate::error::Result;
use crate::models::user::{Verify, VerifyType};

pub async fn create(
    pool: &PgPool,
    email: &str,
    verify_type: VerifyType,
    code_hash: &str,
    password_hash: Option<&str>,
    expires_at: OffsetDateTime,
) -> Result<Verify> {
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

pub async fn find(
    pool: &PgPool,
    email: &str,
    verify_type: VerifyType,
) -> Result<Option<Verify>> {
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

pub async fn increment_attempts(
    pool: &PgPool,
    email: &str,
    verify_type: VerifyType,
) -> Result<()> {
    query(
        r#"
        UPDATE verify
        SET attempts = attempts + 1
        WHERE email = $1 AND "type" = $2
        "#,
    )
        .bind(email)
        .bind(verify_type)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn remove(
    pool: &PgPool,
    email: &str,
    verify_type: VerifyType,
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

// todo: housecleaning, to be run by a worker or something
#[allow(dead_code)]
pub async fn delete_expired(pool: &PgPool) -> Result<u64> {
    let count = query(r#"DELETE FROM verify WHERE expires_at < $1"#)
        .bind(OffsetDateTime::now_utc())
        .execute(pool)
        .await?
        .rows_affected();
    Ok(count)
}