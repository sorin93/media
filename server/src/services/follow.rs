use uuid::Uuid;

use crate::db::follow;
use crate::error::Result;
use crate::models::user::User;
use crate::state::AppState;

// Get followers
pub async fn list(state: &AppState, user_id: Uuid, page: i16) -> Result<Vec<User>> {
    let result = follow::list(&state.pool, user_id, page).await?;
    Ok(result)
}

// Follow
pub async fn create(state: &AppState, from_user_id: Uuid, to_user_id: Uuid) -> Result<User> {
    follow::create(&state.pool, from_user_id, to_user_id).await
}

// Unfollow
pub async fn remove(state: &AppState, from_user_id: Uuid, to_user_id: Uuid) -> Result<()> {
    follow::remove(&state.pool, from_user_id, to_user_id).await?;
    Ok(())
}
