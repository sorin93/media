use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Authenticated {
    pub user_id: Uuid,
    pub session_id: Uuid,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid, // user_id
    pub sid: Uuid, // session_id
    pub iat: i64,  // issued at
    pub exp: i64,  // expires at
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct RefreshToken {
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub status: RefreshTokenStatus,
    pub expires_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq)]
#[sqlx(type_name = "refresh_token_status", rename_all = "snake_case")]
#[serde(rename_all = "lowercase")]
pub enum RefreshTokenStatus {
    Active,
    Used,
    Revoked,
}

impl RefreshToken {
    pub fn is_valid(&self) -> bool {
        self.status == RefreshTokenStatus::Active && OffsetDateTime::now_utc() < self.expires_at
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq)]
#[sqlx(type_name = "verify_type", rename_all = "snake_case")]
#[serde(rename_all = "lowercase")]
pub enum VerifyType {
    Register,
    Login,
    Email,
}

#[derive(Clone, FromRow)]
pub struct Verify {
    pub user_id: Option<Uuid>,
    pub r#type: VerifyType,
    pub created_at: OffsetDateTime,
    pub expires_at: OffsetDateTime,
    pub email: String,
    pub code_hash: String,
    pub password_hash: Option<String>,
    pub attempts: i16,
}
