use axum::Json;
use serde_json::{json, Value};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

use crate::db::{refresh_token, user, verify};
use crate::error::{Error, Result};
use crate::models::auth::TokenResponse;
use crate::models::user::{RegisterRequest, VerifyType::Register, Account, User};
use crate::services::{auth, util};
use crate::state::AppState;

pub async fn initiate_registration(
    state: &AppState,
    input: RegisterRequest,
) -> Result<()> {
    if user::exists_by_email(&state.pool, &input.email).await? {
        return Err(Error::Conflict("Email already registered".into()));
    }

    let password_hash = hash_password(&input.password)?;
    let (code, code_hash) = util::generate_code();

    let expires_at = OffsetDateTime::now_utc()
        + Duration::seconds(state.config.verify_code_ttl_seconds);

    verify::create(
        &state.pool,
        &input.email,
        Register,
        &code_hash,
        Some(&password_hash),
        expires_at,
    )
    .await?;

    state.email_client.send_verify_code(&input.email, &code).await?;
    Ok(())
}

pub async fn complete_registration(
    state: &AppState,
    input: RegisterRequest,
) -> Result<(User, TokenResponse)> {
    
    // We unwrap code here because the Handler guarantees it exists to get here
    let code = input.code.ok_or(Error::BadRequest("Code required".into()))?; 
    let verify_type = Register;

    let pending = verify::find(&state.pool, &input.email, verify_type.clone())
        .await?
        .ok_or(Error::CodeInvalid)?;

    // ... validation logic (attempts, expiry) ...
    if pending.attempts >= 5 { return Err(Error::TooManyAttempts); }
    if pending.expires_at < OffsetDateTime::now_utc() {
        verify::remove(&state.pool, &input.email, verify_type).await?;
        return Err(Error::CodeExpired);
    }

    // Verify Code
    let code_hash = util::hash(&code);
    if code_hash != pending.code_hash {
        verify::increment_attempts(&state.pool, &input.email, verify_type).await?;
        return Err(Error::CodeInvalid);
    }

    // Double check user didn't get created by someone else in the meantime
    if user::exists_by_email(&state.pool, &input.email).await? {
        verify::remove(&state.pool, &input.email, verify_type).await?;
        return Err(Error::Conflict("Email already registered".into()));
    }

    // Retrieve the password hash we saved in step 1
    let password_hash = pending
        .password_hash
        .ok_or_else(|| Error::Internal("Missing password hash".into()))?;

    // CREATE THE USER
    let user = user::create(&state.pool, &input.name, &input.email, &password_hash, input.phone, &input.country, input.birth_date).await?;

    // Cleanup
    verify::remove(&state.pool, &input.email, verify_type).await?;

    let tokens = auth::create_token_pair(state, user.user_id).await?;

    Ok((user, tokens))
}

pub async fn find_account_by_id(state: &AppState, user_id: Uuid) -> Result<Account> {
    user::find_account_by_id(&state.pool, user_id)
        .await?
        .ok_or_else(|| Error::NotFound("User not found".into()))
}

pub async fn find_user_by_id(state: &AppState, user_id: Uuid) -> Result<Json<Value>> {
    let user = user::find_user_by_id(&state.pool, user_id).await?;
    Ok(Json(json!({ "user": user })))
}

pub async fn update_password(
    state: &AppState,
    user_id: Uuid,
    current_password: &str,
    new_password: &str,
) -> Result<User> {
    let user = find_account_by_id(state, user_id).await?;

    verify_password(current_password, &user.password_hash)?;

    let password_hash = hash_password(new_password)?;
    let user = user::update_password(&state.pool, user_id, &password_hash).await?;

    refresh_token::revoke_all_for_user(&state.pool, user_id).await?;

    Ok(user)
}

pub async fn remove(state: &AppState, user_id: Uuid) -> Result<()> {
    user::remove(&state.pool, user_id).await?;
    Ok(())
}

fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| Error::Internal("Failed to hash password".into()))?;
    Ok(hash.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> Result<()> {
    let parsed_hash =
        PasswordHash::new(hash).map_err(|_| Error::Internal("Invalid password hash".into()))?;
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| Error::Unauthorized("Invalid credentials".into()))
}