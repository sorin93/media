use crate::error::Result;
use crate::history::model::HistoryType;
use crate::history::repository;
use crate::state::AppState;

pub async fn add_to_history(
    state: &AppState,
    user_id: uuid::Uuid,
    target_id: uuid::Uuid,
    history_type: HistoryType,
) -> Result<()> {
    repository::create(&state.pool, user_id, target_id, history_type).await
}
