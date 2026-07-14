use crate::comment::model::{Comment, CreateComment};
use crate::error::Result;
use crate::state::AppState;
use uuid::Uuid;

pub async fn create(
    state: &AppState,
    user_id: Uuid,
    media_id: Uuid,
    content: &str,
) -> Result<Comment> {
    crate::comment::repository::create(&state.pool, user_id, media_id, content).await
}

pub async fn list_by_media(state: &AppState, media_id: Uuid, page: i16) -> Result<Vec<Comment>> {
    crate::comment::repository::list_by_media(&state.pool, media_id, page).await
}
