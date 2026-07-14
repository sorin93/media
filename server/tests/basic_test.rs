// Basic tests to get started with testing
// These tests verify that your code compiles and basic functionality works

use media::config::Config;

#[tokio::test]
async fn test_config_creation() {
    // Test that configuration can be loaded
    dotenv::dotenv().ok();
    let config = Config::from_env();

    // Verify config has expected fields
    assert!(
        !config.database_url.is_empty(),
        "Database URL should not be empty"
    );
}

#[tokio::test]
async fn test_app_state_creation() {
    // Test that AppState can be created
    dotenv::dotenv().ok();
    let config = Config::from_env();

    // Create a database pool
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to database");

    // This test just verifies AppState::new() works
    let _state = media::state::AppState::new(config, pool).await;

    // If we get here without panicking, the test passes
    assert!(true);
}

#[test]
fn test_modules_compile() {
    // This test verifies that all your modules compile correctly
    // If this test runs, it means the code compiles
    assert!(true);
}
