use uuid::Uuid;

use crate::error::Result;
use crate::state::AppState;

pub async fn create(state: &AppState, user_id: Uuid, media_id: Uuid) -> Result<()> {
    crate::like::repository::create(&state.pool, user_id, media_id).await
}

pub async fn remove(state: &AppState, user_id: Uuid, media_id: Uuid) -> Result<()> {
    crate::like::repository::remove(&state.pool, user_id, media_id).await
}
