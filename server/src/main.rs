use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use media::{api, config, state}; 

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Config
    dotenv::dotenv().ok();
    let config = config::Config::from_env();

    // Postgres
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await?;
    tracing::info!("Postgres connection pool created successfully");

    // App State
    let state = state::AppState::new(config, pool).await;

    // Axum Api Server
    let listener = TcpListener::bind("[::]:4443").await?;
    let addr = listener.local_addr()?;
    tracing::info!("Axum server listening on {}", addr);
    let app = api::app(state);
    axum::serve(listener, app).await?;

    Ok(())
}