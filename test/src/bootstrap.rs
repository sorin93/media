use axum::{Router, middleware as axum_middleware};
use tokio::net::TcpListener;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};
use tracing_subscriber::prelude::*;

use crate::config::Config;
use crate::core::middleware::{cors_layer, security_headers};
use crate::state::AppState;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Config
    dotenv::dotenv().ok();
    let config = Config::from_env();

    // Postgres
    let pool = crate::db::create_pool(&config).await?;

    // Run migrations
    crate::db::run_migrations(&pool).await?;

    // App State
    let state = AppState::new(config, pool).await;

    // Create router with all feature routes
    // Middleware order (from innermost to outermost):
    // 1. inject_state (adds AppState to request extensions)
    // 2. security_headers (adds security headers)
    // 3. cors_layer (handles CORS preflight)
    // 4. CompressionLayer (compresses responses)
    // 5. TraceLayer (logs requests - outermost)
    let app = Router::new()
        // Merge routes from all features
        .merge(crate::auth::http::routes())
        .merge(crate::user::http::routes())
        .merge(crate::media::http::routes())
        .merge(crate::follow::http::routes())
        .merge(crate::comment::http::routes())
        .merge(crate::like::http::routes())
        // Apply global middleware (ORDER MATTERS! - from bottom to top)
        .layer(axum_middleware::from_fn_with_state(
            state.clone(),
            crate::auth::middleware::inject_state,
        ))
        .layer(axum_middleware::from_fn(security_headers))
        .layer(cors_layer())
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // Axum Api Server
    let listener = TcpListener::bind("[::]:4443").await?;
    let addr = listener.local_addr()?;
    tracing::info!("Axum server listening on {}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}
