use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, PartialEq, sqlx::Type)]
#[sqlx(type_name = "refresh_token_status", rename_all = "snake_case")]
#[serde(rename_all = "lowercase")]
pub enum RefreshTokenStatus {
    Active,
    Used,
    Revoked,
}

#[derive(sqlx::FromRow)]
pub struct RefreshToken {
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub status: RefreshTokenStatus,
    pub expires_at: OffsetDateTime,
}

impl RefreshToken {
    pub fn is_valid(&self) -> bool {
        match self.status {
            RefreshTokenStatus::Active => self.expires_at > OffsetDateTime::now_utc(),
            _ => false,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid, // user_id
    pub sid: Uuid, // session_id (refresh_table & SSE connections)
    pub exp: i64,
    pub iat: i64,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

// Helper types for extractors
#[derive(Clone)]
pub struct Authenticated {
    pub user_id: Uuid,
    pub session_id: Uuid,
}

#[derive(Clone)]
pub struct OptionalAuth(pub Option<Authenticated>);
