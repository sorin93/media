# Integration Tests

This directory contains integration tests for the Media server.

## Test Structure

- `integration_test.rs` - Basic integration tests for core functionality
- `api_test.rs` - Tests for API endpoints (auth, users, etc.)
- `common.rs` - Shared test utilities and helpers

## Running Tests

### Run all tests
```bash
cd server
cargo test
```

### Run specific test file
```bash
cargo test --test integration_test
cargo test --test api_test
```

### Run a specific test function
```bash
cargo test test_health_check
cargo test test_register_endpoint_exists
```

### Run tests with output
```bash
cargo test -- --nocapture
```

## Important Notes

1. **Database**: These tests connect to a real database. Make sure to:
   - Set up a test database (different from production)
   - Update `.env` with test database credentials
   - Run migrations on the test database

2. **Test Isolation**: Each test should be independent. Consider:
   - Using transactions that rollback after each test
   - Creating test-specific data
   - Cleaning up after tests

3. **Environment**: Tests load `.env` file. Create a `.env.test` for test-specific config.

## Writing New Tests

1. Create a new test function with `#[tokio::test]`
2. Use the `common` module for shared utilities
3. Test one thing per test function
4. Use descriptive test names

Example:
```rust
#[tokio::test]
async fn test_user_registration_success() {
    // Arrange
    let app = create_test_app().await;
    
    // Act
    let response = app
        .oneshot(
            Request::builder()
                .uri("/auth/register")
                .method("POST")
                .body(Body::from(json!({...}).to_string()))
        )
        .await
        .unwrap();
    
    // Assert
    assert_eq!(response.status(), StatusCode::CREATED);
}
```
