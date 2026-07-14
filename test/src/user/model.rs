use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use time::Date;
use uuid::Uuid;

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

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub phone: Option<String>,
    pub country: String,
    pub birth_date: Date,
    pub code: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdatePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}
