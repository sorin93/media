// Minimal test utilities for follow tests

use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use tower::ServiceExt;
use uuid::Uuid;

use media::config::Config;
use media::state::AppState;

// Create a test app with the full API routes
pub async fn create_test_app() -> Router {
    dotenv::dotenv().ok();
    let config = Config::from_env();

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to database");

    let state = AppState::new(config, pool).await;

    // Use the full app from api::mod to get all routes and middleware
    media::api::app(state)
}

// Helper to make authenticated requests
pub async fn make_authenticated_request(
    app: &mut Router,
    method: &str,
    uri: &str,
    token: &str,
) -> axum::http::Response<Body> {
    let request = Request::builder()
        .uri(uri)
        .method(method)
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();

    app.oneshot(request).await.unwrap()
}

// Helper to parse response body as JSON
pub async fn parse_json_response(response: axum::http::Response<Body>) -> serde_json::Value {
    let body = response.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&body).unwrap()
}
