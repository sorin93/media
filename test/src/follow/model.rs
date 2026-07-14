use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateFollow {
    pub target_user_id: Uuid,
}

#[derive(Deserialize)]
pub struct FollowParams {
    pub page: Option<i16>,
}
