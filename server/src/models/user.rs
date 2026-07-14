use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use time::{Date, OffsetDateTime};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, Type, PartialEq)]
#[sqlx(type_name = "verify_type", rename_all = "snake_case")]
#[serde(rename_all = "lowercase")]
pub enum VerifyType {
    Register,
    Login,
    Email,
}

#[derive(Clone, Serialize, FromRow)]
pub struct Account {
    pub user_id: Uuid,
    pub media_id: Option<Uuid>,
    pub name: String,
    pub password_hash: String,
    pub email: String,
    pub country: String,
}

#[derive(Clone, Deserialize, Serialize, FromRow)]
pub struct User {
    pub user_id: Uuid,
    pub media_id: Option<Uuid>,
    pub name: String,
}

impl From<Account> for User {
    fn from(user: Account) -> Self {
        Self {
            user_id: user.user_id,
            media_id: user.media_id,
            name: user.name,
        }
    }
}

#[derive(Clone, FromRow)]
pub struct Verify {
    #[allow(dead_code)]
    pub user_id: Option<Uuid>,
    #[allow(dead_code)]
    pub r#type: VerifyType,
    #[allow(dead_code)]
    pub created_at: OffsetDateTime,
    pub expires_at: OffsetDateTime,
    #[allow(dead_code)]
    pub email: String,
    pub code_hash: String,
    pub password_hash: Option<String>,
    pub attempts: i16,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub phone: Option<String>,
    pub country: String,
    pub birth_date: Date, // todo: why received date is wrong?
    pub code: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdatePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}