// Feature modules (business domains) - FEATURE-FIRST ARCHITECTURE
pub mod auth;
pub mod comment;
pub mod follow;
pub mod history;
pub mod like;
pub mod media;
pub mod text;
pub mod user;

// Infrastructure modules
pub mod clients;
pub mod config;
pub mod db; // Database infrastructure (pool, migrations)
pub mod state;

// Core utilities (cross-cutting concerns)
pub mod core; // Contains: crypto, time, validation, middleware (CORS, security headers)

// Error handling
pub mod error;

// Bootstrap (initialization logic)
pub mod bootstrap;
