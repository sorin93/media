use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

use crate::auth::model::{
    Authenticated, Claims, LoginRequest, RefreshRequest, RefreshTokenStatus, TokenResponse,
};
use crate::auth::repository;
use crate::core::crypto::{generate_token, hash};
use crate::error::{Error, Result};
use crate::state::AppState;
use crate::user::model::Account;
use crate::user::repository as user_repository;

pub async fn create(state: &AppState, input: LoginRequest) -> Result<(Account, TokenResponse)> {
    let user = user_repository::find_by_email(&state.pool, &input.email)
        .await?
        .ok_or_else(|| Error::Unauthorized("Invalid credentials".into()))?;

    crate::user::service::verify_password(&input.password, &user.password_hash)?;
    let tokens = create_token_pair(state, user.user_id).await?;
    Ok((user, tokens))
}

pub async fn refresh(state: &AppState, input: RefreshRequest) -> Result<(Account, TokenResponse)> {
    let token_hash = hash(&input.refresh_token);
    let stored = repository::find_refresh_token_by_hash(&state.pool, &token_hash)
        .await?
        .ok_or(Error::TokenInvalid)?;

    match stored.status {
        RefreshTokenStatus::Revoked => return Err(Error::TokenInvalid),
        RefreshTokenStatus::Used => {
            tracing::warn!(session = %stored.session_id, "Reuse detected! Revoking session.");
            repository::revoke_session(&state.pool, stored.session_id).await?;
            return Err(Error::TokenReused);
        }
        RefreshTokenStatus::Active => {
            if !stored.is_valid() {
                return Err(Error::TokenExpired);
            }
        }
    }

    let user = crate::user::service::find_account_by_id(state, stored.user_id).await?;

    // Rotation: Used -> New Token (Same Session)
    repository::mark_token_used(&state.pool, &token_hash).await?;
    let tokens = create_token_pair_with_session(state, stored.user_id, stored.session_id).await?;

    Ok((user, tokens))
}

pub async fn remove(state: &AppState, session_id: Uuid) -> Result<()> {
    repository::revoke_session(&state.pool, session_id).await?;
    Ok(())
}

pub fn validate_access_token(state: &AppState, token: &str) -> Result<Authenticated> {
    let key = DecodingKey::from_secret(state.config.jwt_secret.as_bytes());
    let validation = Validation::default();
    let token_data = decode::<Claims>(token, &key, &validation)?;
    Ok(Authenticated {
        user_id: token_data.claims.sub,
        session_id: token_data.claims.sid,
    })
}

pub async fn create_token_pair(state: &AppState, user_id: Uuid) -> Result<TokenResponse> {
    let session_id = Uuid::new_v4();
    create_token_pair_with_session(state, user_id, session_id).await
}

async fn create_token_pair_with_session(
    state: &AppState,
    user_id: Uuid,
    session_id: Uuid,
) -> Result<TokenResponse> {
    let config = &state.config;
    let access_token = create_access_token(user_id, session_id, config)?;
    let (refresh_token, token_hash) = generate_token();
    let expires_at =
        OffsetDateTime::now_utc() + Duration::seconds(config.refresh_token_ttl_seconds);

    repository::insert_refresh_token(&state.pool, &token_hash, user_id, session_id, expires_at)
        .await?;

    Ok(TokenResponse {
        access_token,
        refresh_token,
        token_type: "Bearer".into(),
        expires_in: config.access_token_ttl_seconds,
    })
}

fn create_access_token(
    user_id: Uuid,
    session_id: Uuid,
    config: &crate::config::Config,
) -> Result<String> {
    let now = OffsetDateTime::now_utc();
    let exp = now + Duration::seconds(config.access_token_ttl_seconds);
    let claims = Claims {
        sub: user_id,
        sid: session_id,
        iat: now.unix_timestamp(),
        exp: exp.unix_timestamp(),
    };
    let key = EncodingKey::from_secret(config.jwt_secret.as_bytes());
    encode(&Header::default(), &claims, &key)
        .map_err(|_| Error::Internal("Token creation failed".into()))
}
