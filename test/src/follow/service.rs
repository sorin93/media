use uuid::Uuid;

use crate::error::Result;
use crate::state::AppState;

pub async fn create(state: &AppState, user_id: Uuid, target_user_id: Uuid) -> Result<()> {
    crate::follow::repository::create(&state.pool, user_id, target_user_id).await
}

pub async fn remove(state: &AppState, user_id: Uuid, target_user_id: Uuid) -> Result<()> {
    crate::follow::repository::remove(&state.pool, user_id, target_user_id).await
}
