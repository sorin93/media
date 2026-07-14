use pgvector::Vector;
use uuid::Uuid;

use crate::clients::cloudflare;
use crate::db::media;
use crate::error::Result;
use crate::models::media::{CreateMediaResponse, Media, MediaType, Search};
use crate::state::AppState;

pub async fn list(
    state: &AppState,
    user_id: Uuid,
    search: Search,
    page: i16,
) -> Result<Vec<Media>> {
    let result = media::list(&state.pool, user_id, search, page).await?;
    Ok(result)
}

pub async fn create(
    state: &AppState,
    user_id: Uuid,
    parent_media_id: Option<Uuid>,
    r#type: MediaType,
    caption: Option<String>,
) -> Result<CreateMediaResponse> {
    let media_id = media::create(&state.pool, user_id, parent_media_id, r#type, caption).await?;
    let path = format!("{}/original", media_id);
    let upload_url = cloudflare::create_presigned_upload_url(&state.cloudflare_bucket, &path).await?;
    Ok(CreateMediaResponse { media_id, upload_url })
}

pub async fn activate(
    state: &AppState,
    media_id: Uuid,
    embedding: Vector,
    blurhash: String,
) -> Result<Media> {
    let result = media::activate(&state.pool, media_id, embedding, blurhash).await?;
    Ok(result)
}

pub async fn update(
    state: &AppState,
    user_id: Uuid,
    media_id: Uuid,
    caption: Option<String>,
) -> Result<Media> {
    // Keep the current embedding, so I don't have to download the AVIF and convert it to JPEG and resize again
    let result = media::update(&state.pool, user_id, media_id, caption).await?;
    Ok(result)
}

pub async fn remove(
    state: &AppState,
    user_id: Uuid,
    media_id: Uuid,
) -> Result<()> {
    media::remove(&state.pool, user_id, media_id).await?;
    Ok(())
}