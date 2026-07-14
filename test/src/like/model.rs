use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct LikeParams {
    pub page: Option<i16>,
}
