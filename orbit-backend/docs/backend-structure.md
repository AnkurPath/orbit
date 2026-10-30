# Backend Folder Structure

This backend is organized as a Rust workspace. The goal is to move quickly in v1 while keeping clear module boundaries so individual pieces can be scaled or extracted later.

## Top Level

```txt
orbit-backend/
  Cargo.toml
  services/
  crates/
  migrations/
  compose.yaml
  docker/
  configs/
  docs/
```

### `Cargo.toml`

Workspace manifest. It lists every backend service and shared crate.

Use it to run commands across the workspace:

```sh
cargo check --workspace
cargo test --workspace
```

### `services/`

Deployable applications live here. Each service can become its own container and can be scaled independently.

For v1, keep product modules inside `services/api` unless a module has a real operational reason to split out.

### `crates/`

Shared Rust libraries live here. These crates should contain reusable code, not product-specific HTTP route wiring.

Keep these crates small and boring. A shared crate should exist because multiple services need the code.

### `migrations/`

Database schema migrations.

Recommended convention:

```txt
0001_init.sql
0002_auth.sql
0003_orgs.sql
0004_docs.sql
0005_attendance.sql
```

Prefer module-owned schemas or tables. For example, `docs.pages`, `attendance.entries`, or table prefixes like `docs_pages` and `attendance_entries`.

### `compose.yaml`

Local development dependencies for the backend application.

It starts:

- `postgres`: primary application database
- `redis`: cache, queues, rate limits, and future presence/realtime state
- `minio`: S3-compatible object storage for document assets and uploads
- `mailpit`: local SMTP server and email inbox for development

The Rust API is intentionally not included yet. During early development, run it directly with:

```sh
cargo run -p orbit-api
```

Add API and worker containers later when the Dockerfiles are production-ready.

### `docker/`

Dockerfiles for deployable services.

Expected files:

```txt
Dockerfile.api
Dockerfile.worker
```

Add `Dockerfile.realtime` later if you split realtime features into a separate service.

### `configs/`

Environment-specific config files.

Examples:

```txt
local.toml
production.toml
```

Do not commit secrets here. Store secrets in the deployment platform or a secret manager.

### `docs/`

Engineering notes for architecture, database design, APIs, and operating procedures.

Current docs:

```txt
backend-structure.md
api-docs.md
```

## Services

```txt
services/
  api/
  worker/
```

### `services/api`

Main Axum HTTP API. This is the v1 modular monolith.

```txt
services/api/src/
  main.rs
  app.rs
  state.rs
  config/
  http/
  modules/
  openapi.rs
  db/
```

#### `main.rs`

Process entry point. It loads config, initializes telemetry, builds app state, starts the HTTP server, and handles shutdown.

#### `app.rs`

Builds the Axum app/router. This keeps `main.rs` small.

#### `state.rs`

Shared application state passed into handlers, such as database pool, config, cache clients, and service dependencies.

#### `openapi.rs`

OpenAPI document definition for the API.

Handlers are documented with `#[utoipa::path(...)]`, DTOs are documented with `#[derive(ToSchema)]`, and Swagger UI is served at `/swagger-ui`.

Read `docs/api-docs.md` for the full workflow for documenting new handlers.

#### `config/`

API configuration loading and validation.

Use this for environment variables, config files, and strongly typed runtime settings.

#### `http/`

HTTP-specific concerns:

```txt
http/
  router.rs
  error.rs
  response.rs
  middleware/
```

- `router.rs`: combines module routers into one API router.
- `error.rs`: maps application errors to HTTP responses.
- `response.rs`: common response envelopes and helpers.
- `middleware/`: auth, CORS, tracing, request IDs, rate limiting.

#### `modules/`

Product and platform modules. Each module owns its HTTP routes, business logic, database access, DTOs, and internal models.

Current v1 modules:

```txt
modules/
  auth/
  orgs/
  users/
  docs/
  attendance/
```

Recommended module layout:

```txt
routes.rs       # Axum handlers and route definitions
service.rs      # business logic
repository.rs   # database queries
models.rs       # internal domain models
dto.rs          # request and response types
mod.rs          # module exports
```

Keep cross-module calls explicit through service APIs. Avoid directly reaching into another module's repository unless there is a strong reason.

#### `modules/auth`

Authentication and identity module.

Owns login/session/token integration points, passwordless or SSO hooks, auth middleware helpers, and user identity mapping.

#### `modules/orgs`

Organization and workspace module.

Owns organizations, memberships, roles, invitations, and tenant-level settings.

#### `modules/users`

User profile module.

Owns user profile data that is not authentication-specific, such as display name, avatar, preferences, and profile status.

#### `modules/docs`

Notion-like documents module.

Owns documents, pages, blocks, permissions, document versions, and editor-specific behavior.

Keep collaborative editing internals isolated here so this module can later move toward a dedicated docs service if needed.

Current sample endpoints:

```txt
GET /docs/health
GET /docs/sample-pages
GET /docs/sample-pages/{page_id}
```

These endpoints are temporary examples for OpenAPI and frontend contract exploration. Replace them with real page/block endpoints as the docs product becomes concrete.

#### `modules/attendance`

Attendance tracker module.

Owns attendance policies, check-ins, check-outs, daily summaries, reports, and attendance-specific permissions.

#### `db/`

API-local database glue.

Use this for API-specific migration checks, transaction helpers, or wiring around the shared `crates/db` package.

## Worker

```txt
services/worker/src/
  main.rs
  jobs/
```

### `services/worker`

Background worker process. It should be deployed separately from the API so background load does not affect request latency.

Good worker responsibilities:

- document indexing
- attendance report generation
- notification fanout
- cleanup jobs
- scheduled rollups

### `jobs/`

Each background job gets a focused file.

Examples:

```txt
document_indexing.rs
attendance_reports.rs
notifications.rs
```

## Shared Crates

```txt
crates/
  domain/
  db/
  infra/
  telemetry/
```

### `crates/domain`

Pure shared domain types.

Good contents:

- strongly typed IDs
- time helpers
- permissions
- domain events

Avoid dependencies on Axum, SQLx, Redis, HTTP, or deployment infrastructure.

### `crates/db`

Shared database utilities.

Good contents:

- PostgreSQL pool creation
- transaction helpers
- pagination helpers
- common SQLx types

Avoid product-specific queries here. Product queries belong in module repositories.

### `crates/infra`

Shared infrastructure clients and adapters.

Good contents:

- Redis
- object storage
- email
- queue clients

Keep vendor-specific details hidden behind small interfaces where useful.

### `crates/telemetry`

Logging, tracing, and metrics setup shared by services.

Good contents:

- tracing subscriber setup
- log formatting
- metrics initialization
- request tracing helpers

## Future Additions

Add these only when the product needs them:

```txt
services/realtime/
crates/contracts/
```

### `services/realtime`

Use this for WebSocket-heavy features like chat, live document collaboration, presence, and typing indicators.

This is a likely future split because realtime traffic scales differently from normal HTTP API traffic.

### `crates/contracts`

Use this if multiple services need shared API DTOs or event contracts.

Do not add it too early. Shared contracts are useful, but they can also create coupling if every type gets placed there by default.

