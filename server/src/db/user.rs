use sqlx::{PgPool, query, query_as};
use time::{Date, OffsetDateTime};
use uuid::Uuid;

use crate::error::Result;
use crate::models::user::{Account, User};

// todo: phone, birth_date
pub async fn create(
    pool: &PgPool,
    name: &str,
    email: &str,
    password_hash: &str,
    phone: Option<String>,
    country: &str,
    birth_date: Date,
) -> Result<User> {
    let result = query_as(
        r#"
        INSERT INTO "user" (name, email, password_hash, phone, country, birth_date)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING user_id, media_id, name
        "#,
    )
        .bind(name)
        .bind(email)
        .bind(password_hash)
        .bind(phone)
        .bind(country)
        .bind(birth_date)
        .fetch_one(pool)
        .await?;
    Ok(result)
}

pub async fn find_user_by_id(pool: &PgPool, user_id: Uuid) -> Result<User> {
    let result = query_as(r#"SELECT user_id, media_id, name FROM "user" WHERE user_id = $1"#)
        .bind(user_id)
        .fetch_one(pool)
        .await?;
    Ok(result)
}

pub async fn find_account_by_id(pool: &PgPool, user_id: Uuid) -> Result<Option<Account>> {
    let result = query_as(
        r#"
        SELECT user_id, media_id, name, password_hash, email, country
        FROM "user"
        WHERE user_id = $1
        "#
    )
        .bind(user_id)
        .fetch_optional(pool)
        .await?;
    Ok(result)
}

pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<Account>> {
    let result = query_as(
        r#"
        SELECT user_id, media_id, name, password_hash, email, country
        FROM "user"
        WHERE email = $1
    "#)
        .bind(email)
        .fetch_optional(pool)
        .await?;
    Ok(result)
}

pub async fn exists_by_email(pool: &PgPool, email: &str) -> Result<bool> {
    let result: (bool,) =
        query_as(r#"SELECT EXISTS(SELECT 1 FROM "user" WHERE email = $1)"#)
            .bind(email)
            .fetch_one(pool)
            .await?;
    Ok(result.0)
}

pub async fn update_password(
    pool: &PgPool,
    user_id: Uuid,
    password_hash: &str,
) -> Result<User> {
    let result = query_as(
        r#"
        UPDATE "user"
        SET password_hash = $2, updated_at = $3
        WHERE user_id = $1
        RETURNING user_id, media_id, name
        "#,
    )
        .bind(user_id)
        .bind(password_hash)
        .bind(OffsetDateTime::now_utc())
        .fetch_one(pool)
        .await?;
    Ok(result)
}

pub async fn remove(pool: &PgPool, user_id: Uuid) -> Result<()> {
    query(r#"DELETE FROM "user" WHERE user_id = $1"#)
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}