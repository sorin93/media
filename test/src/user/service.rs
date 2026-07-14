use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

use crate::error::{Error, Result};
use crate::state::AppState;
use crate::user::model::Account;
use uuid::Uuid;

pub fn verify_password(password: &str, password_hash: &str) -> Result<()> {
    let parsed_hash = PasswordHash::new(password_hash)
        .map_err(|_| Error::Internal("Invalid password hash format".into()))?;
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| Error::Unauthorized("Invalid credentials".into()))
}

pub fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| Error::Internal("Failed to hash password".into()))?;
    Ok(hash.to_string())
}

pub async fn find_account_by_id(state: &AppState, user_id: Uuid) -> Result<Account> {
    crate::user::repository::find_account_by_id(&state.pool, user_id)
        .await?
        .ok_or_else(|| Error::NotFound("User not found".into()))
}
