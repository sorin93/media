pub mod auth;
pub mod comment;
pub mod extractors;
pub mod follow;
pub mod media;
pub mod middleware;
pub mod user;

use axum::{Router, middleware as axum_middleware};
use tower_http::{compression::CompressionLayer, trace::TraceLayer};

use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .merge(auth::routes())
        .merge(comment::routes())
        .merge(follow::routes())
        .merge(media::routes())
        .merge(user::routes())
}

pub fn app(state: AppState) -> Router {
    routes()
        .layer(CompressionLayer::new())
        .layer(middleware::cors_layer())
        .layer(axum_middleware::from_fn(middleware::security_headers))
        .layer(axum_middleware::from_fn_with_state(
            state.clone(),
            middleware::inject_state,
        ))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
