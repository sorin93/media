use serde::{Deserialize, Serialize};
use sqlx::Type;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[sqlx(type_name = "history_type", rename_all = "snake_case")]
#[serde(rename_all = "lowercase")]
pub enum HistoryType {
    Text,
    Media,
}
