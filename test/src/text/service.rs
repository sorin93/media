use crate::error::Result;
use crate::state::AppState;
use crate::text::model::Text;
use crate::text::repository;

pub async fn find_or_create_text(state: &AppState, text: String) -> Result<Text> {
    // Check cache first
    if let Some(cached) = repository::find_by_text(&state.pool, &text).await? {
        return Ok(cached);
    }

    // TODO: Call embedding API and create new text
    // For now, return an error
    Err(crate::error::Error::Internal(
        "Text not found and embedding not implemented".into(),
    ))
}

pub async fn search_similar_texts(
    state: &AppState,
    embedding: pgvector::Vector,
    page: i16,
) -> Result<Vec<Text>> {
    repository::list_by_embedding(&state.pool, embedding, page).await
}
