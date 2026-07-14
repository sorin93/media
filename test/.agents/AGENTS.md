# Workspace Rules

## Environment & Command Execution
- Do not run build, installation, or compilation commands (such as `cargo run`, `cargo build`, `cargo check`) as these toolchains are not available in the agent's sandbox (sandbox cargo is different than toolchains cargo). Rely on the user to run builds and report any compilation or lint errors.

## Architecture & Layers
Strict flow: **API -> Services -> [DB / Clients]**.

- `src/api/`      : Axum handlers (Request/Response only).
- `src/services/` : Orchestration & Business logic. Calls DB/Clients.
- `src/db/`       : Postgres Database logic.
- `src/clients/`  : External API integrations (e.g., Vertex AI).
- `src/models/`   : Data Structs (do not define structs in api, services, clients, or db directly).
- `src/error.rs`  : Centralized Error and Result definitions.
- `src/state.rs`  : App State.
- `src/db.sql`    : Full Postgres Database Schema Definitions

## Domain Module Pattern
Logic is grouped by **Domain Sections** (e.g., user, media, document, comment). 
Each section has its own file within the layer folders. 
Example: `src/api/user.rs` calls `src/services/user.rs` which calls `src/db/user.rs`.

## Error Handling Rules
- **No `unwrap()` or `expect()`**: Always propagate errors.
- **DB Layer**: Functions in `src/db/` must return `Result<T, sqlx::Error>`.
- **All Other Layers**: Must return the custom `crate::error::Result<T>`.
- **Implementation**: All custom error types are defined in `src/error.rs`.

## Implementation Flow
- **API** handlers must never call **DB** or **Clients** directly; they must go through **Services**.
- Services are responsible for orchestrating data between **DB** and **Clients**.

## Workers
- Spawned Workers started in `src/main.rs` belong in `src/services/<worker_name>.rs` (any structs they use still place in `src/models`).

