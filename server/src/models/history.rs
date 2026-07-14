use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Clone, Copy, Deserialize, Serialize, Type)]
#[sqlx(type_name = "history_type", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum HistoryType {
    Image,
    Video,
    Text,
    Similar,
    User,
}