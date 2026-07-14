use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, patch, post},
};
use pgvector::Vector;
use serde_json::{Value, json};
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::models::auth::{Authenticated, OptionalAuth};
use crate::models::media::{
    ActivateMedia, CreateMedia, MediaQuery, MediaType::Image, Search2, UpdateMedia,
};
use crate::models::util::Pagination;
use crate::services::util::{validate_number, validate_number_opt, validate_string_opt};
use crate::services::{comment, like, media};
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/explore", get(explore))
        .route("/following", get(following))
        .route("/search", get(search))
        .route("/media", get(list))
        .route("/media/{media_id}", get(find_by_id))
        .route("/media/{media_id}/suggestions", get(media_suggestions))
        .route("/media/{media_id}/comments", get(media_comments))
        .route("/media/{media_id}/likes", get(media_likes))
        .route("/media/{media_id}/user", get(user_media))
        .route("/media", post(create))
        .route("/media/{media_id}", post(activate))
        .route("/media/{media_id}", patch(update))
        .route("/media/{media_id}", delete(remove))
}

async fn explore(
    State(state): State<AppState>,
    Query(query): Query<Pagination>,
) -> Result<Json<Value>> {
    let page = query.page.unwrap_or(1);
    validate_number(page, 1, 10, "Invalid page")?;
    let json = media::explore(&state, page).await?;
    Ok(json)
}

async fn following(
    State(state): State<AppState>,
    auth: Authenticated,
    Query(query): Query<Pagination>,
) -> Result<Json<Value>> {
    let page = query.page.unwrap_or(1);
    validate_number(page, 1, 10, "Invalid page")?;
    let json = media::following(&state, auth.user_id, page).await?;
    Ok(json)
}

async fn search(
    State(state): State<AppState>,
    auth: OptionalAuth,
    Query(query): Query<Search2>,
) -> Result<Json<Value>> {
    let user_id = auth.0.as_ref().map(|a| a.user_id);
    let page = query.page.unwrap_or(1);
    validate_number(page, 1, 10, "Invalid page")?;
    let json = media::search(&state, user_id, query.query, page).await?;
    Ok(json)
}

async fn media_suggestions(
    State(state): State<AppState>,
    Path((media_id,)): Path<(Uuid,)>,
    Query(query): Query<Pagination>,
) -> Result<Json<Value>> {
    validate_number_opt(query.page, 1, 10, "Invalid page")?;
    let page = query.page.unwrap_or(1);
    let expand = query.expand.unwrap_or(false);
    validate_number(page, 1, 10, "Invalid page")?;
    let json = media::suggestions(&state, media_id, page, expand).await?;
    Ok(json)
}

async fn media_comments(
    State(state): State<AppState>,
    Path((media_id,)): Path<(Uuid,)>,
    Query(query): Query<Pagination>,
) -> Result<Json<Value>> {
    let page = query.page.unwrap_or(1);
    let expand = query.expand.unwrap_or(false);
    validate_number(page, 1, 10, "Invalid page")?;
    let json = comment::media_comments(&state, media_id, page, expand).await?;
    Ok(json)
}

async fn media_likes(
    State(state): State<AppState>,
    Path((media_id,)): Path<(Uuid,)>,
    Query(query): Query<Pagination>,
) -> Result<Json<Value>> {
    let page = query.page.unwrap_or(1);
    let expand = query.expand.unwrap_or(false);
    validate_number(page, 1, 10, "Invalid page")?;
    let json = like::media_likes(&state, media_id, page, expand).await?;
    Ok(json)
}

async fn user_media(
    State(state): State<AppState>,
    Path((media_id,)): Path<(Uuid,)>,
    Query(query): Query<Pagination>,
) -> Result<Json<Value>> {
    let page = query.page.unwrap_or(1);
    let expand = query.expand.unwrap_or(false);
    validate_number(page, 1, 10, "Invalid page")?;
    let json = media::user_media_by_media_id(&state, media_id, page, expand).await?;
    Ok(json)
}

async fn list(
    State(state): State<AppState>,
    auth: OptionalAuth,
    Query(input): Query<MediaQuery>,
) -> Result<Json<Value>> {
    let user_id = auth.0.as_ref().map(|a| a.user_id);
    validate_number_opt(input.page, 1, 10, "Invalid page")?;
    let json = media::list(&state, user_id, input.search, input.page.unwrap_or(1)).await?;
    Ok(json)
}

async fn find_by_id(
    State(state): State<AppState>,
    Path((media_id,)): Path<(Uuid,)>,
) -> Result<Json<Value>> {
    let json = media::find_by_id(&state, media_id).await?;
    Ok(json)
}

async fn create(
    State(state): State<AppState>,
    auth: Authenticated,
    Json(mut input): Json<CreateMedia>,
) -> Result<(StatusCode, Json<Value>)> {
    validate_string_opt(&mut input.caption, 2, 100, "Invalid caption")?;
    if input.r#type != Image {
        return Err(Error::BadRequest("Invalid media type".into()));
    }
    let media_id = media::create(
        &state,
        auth.user_id,
        input.parent_media_id,
        input.r#type,
        input.caption,
    )
    .await?;
    Ok((
        StatusCode::CREATED,
        Json(json!({ "media": { "media_id": media_id }})),
    ))
}

async fn activate(
    State(state): State<AppState>,
    auth: Authenticated,
    Path((media_id,)): Path<(Uuid,)>,
    Json(input): Json<ActivateMedia>,
) -> Result<Json<Value>> {
    if input.embedding.len() != 3072 {
        return Err(Error::BadRequest("Invalid embedding".into()));
    }
    let media = media::activate(
        &state,
        auth.user_id,
        media_id,
        Vector::from(input.embedding),
        input.blurhash,
    )
    .await?;
    Ok(Json(json!({ "media": media })))
}

async fn update(
    State(state): State<AppState>,
    auth: Authenticated,
    Path((media_id,)): Path<(Uuid,)>,
    Json(mut input): Json<UpdateMedia>,
) -> Result<Json<Value>> {
    validate_string_opt(&mut input.caption, 2, 100, "Invalid caption")?;
    let media = media::update(&state, auth.user_id, media_id, input.caption).await?;
    Ok(Json(json!({ "media": media })))
}

async fn remove(
    State(state): State<AppState>,
    auth: Authenticated,
    Path((media_id,)): Path<(Uuid,)>,
) -> Result<StatusCode> {
    media::remove(&state, auth.user_id, media_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
