// Database infrastructure module
pub mod pool;
pub mod migrations;

// Re-export for convenience
pub use pool::create_pool;
pub use migrations::run_migrations;
