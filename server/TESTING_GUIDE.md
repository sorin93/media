# Testing Guide for Media Server

## What I've Created

I've set up integration tests for your Media server project. Here's what you need to know:

## Directory Structure

```
server/
├── Cargo.toml          # Updated with test configuration
├── src/
│   └── ...             # Your existing source code
└── tests/              # Integration tests directory
    ├── README.md       # Documentation on how to run tests
    ├── common.rs       # Shared test utilities
    ├── integration_test.rs  # Basic integration tests
    ├── api_test.rs     # API endpoint tests
    └── auth_flow_test.rs    # Complete auth flow test
```

## Test Files Explained

### 1. `common.rs` - Test Utilities
This file contains helper functions that can be used across all test files:
- `test_config()` - Loads test configuration
- `test_db_pool()` - Creates a database connection pool for tests
- `test_app_state()` - Creates the full app state for testing
- `json_body()` - Helper to create JSON request bodies

### 2. `integration_test.rs` - Basic Tests
Contains foundational tests:
- `test_health_check()` - Example of testing a health endpoint
- `test_auth_register_validation()` - Tests input validation

### 3. `api_test.rs` - API Endpoint Tests
Organized by modules (auth, user, etc.):
- Tests for auth routes (register, login)
- Tests for protected endpoints (verifying auth is required)
- Demonstrates how to structure tests by feature

### 4. `auth_flow_test.rs` - Complete Flow Test
Shows a full integration test:
- Registers a user
- Logs in with those credentials
- Verifies duplicate registration fails
- Demonstrates test isolation and cleanup

## How to Run Tests

### Run all tests:
```bash
cd server
cargo test
```

### Run specific test file:
```bash
cargo test --test integration_test
cargo test --test api_test
cargo test --test auth_flow_test
```

### Run a specific test function:
```bash
cargo test test_health_check
cargo test test_register_endpoint_exists
```

### Run tests with detailed output:
```bash
cargo test -- --nocapture
```

## Important: Database Setup

These tests connect to a **real database**. Before running tests:

1. **Create a test database** (separate from production):
   ```sql
   CREATE DATABASE media_test;
   ```

2. **Update your `.env` file** with test database credentials

3. **Run migrations** on the test database:
   ```bash
   sqlx migrate run
   ```

## Test Isolation Best Practices

For proper testing, you should:

1. **Use a separate test database** - Never test against production
2. **Clean up after tests** - Delete test data created during tests
3. **Use transactions** - Wrap tests in transactions that rollback
4. **Mock external services** - Don't call real APIs (S3, OAuth, etc.)

## Next Steps

To make these tests fully work, you'll need to:

1. **Adjust the test code** to match your actual endpoints
   - The tests I created use example endpoints like `/health`, `/auth/register`
   - Update these to match your actual API routes

2. **Set up test database properly**
   - Create a separate test database
   - Configure test-specific environment variables

3. **Add more tests** for other endpoints:
   - Media upload/download
   - Comments
   - Follows
   - User profile updates

## Learning Resources

- [Rust Book - Testing](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Axum Testing](https://docs.rs/axum/latest/axum/#testing)
- [SQLx Testing](https://docs.rs/sqlx/latest/sqlx/#testing)

## Tips for Team Collaboration

1. **Run tests before committing**: `cargo test`
2. **Add tests for new features**: Don't merge code without tests
3. **Use CI/CD**: Set up GitHub Actions to run tests automatically
4. **Test edge cases**: Empty inputs, invalid data, unauthorized access
5. **Keep tests fast**: Use test databases, mock external services

---

**Note**: The test code I've written uses example endpoints and may need adjustment to match your actual API structure. Check your `src/api/` modules to see the actual route paths and adjust the test URIs accordingly.
