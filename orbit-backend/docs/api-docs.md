# API Documentation

The API publishes OpenAPI documentation from the Axum code using `utoipa`.

This gives the project a FastAPI-like workflow:

- handlers are annotated with `#[utoipa::path(...)]`
- request and response DTOs derive `ToSchema`
- Swagger UI is served by the API
- the raw OpenAPI JSON is available for frontend tooling and client generation

## Local URLs

Start the API:

```sh
cargo run -p orbit-api
```

Then open:

```txt
http://127.0.0.1:8080/swagger-ui
```

Raw OpenAPI JSON:

```txt
http://127.0.0.1:8080/api-docs/openapi.json
```

If port `8080` is already in use:

```sh
ORBIT_API_PORT=18080 cargo run -p orbit-api
```

Then use:

```txt
http://127.0.0.1:18080/swagger-ui
```

## Current Sample Endpoints

The docs module includes sample endpoints so Swagger UI has realistic data while the real document schema is still being built.

```txt
GET /health
GET /docs/health
GET /docs/sample-pages
GET /docs/sample-pages/{page_id}
```

Sample response from `GET /docs/sample-pages`:

```json
[
  {
    "id": "page_demo_project_plan",
    "workspace_id": "workspace_orbit_demo",
    "title": "Orbit v1 Product Plan",
    "blocks": [
      {
        "id": "block_heading_intro",
        "block_type": "heading",
        "text": "Build the docs and attendance foundation",
        "position": 1
      },
      {
        "id": "block_paragraph_scope",
        "block_type": "paragraph",
        "text": "Version one focuses on Notion-like pages, block editing, and attendance tracking.",
        "position": 2
      },
      {
        "id": "block_todo_schema",
        "block_type": "todo",
        "text": "Design docs_pages and docs_blocks tables.",
        "position": 3
      }
    ]
  }
]
```

## Where OpenAPI Code Lives

```txt
services/api/src/openapi.rs
```

This file defines the API document:

- API title, version, and description
- documented route handlers
- shared schema components
- tags shown in Swagger UI

When you add a new documented endpoint, register the handler in `paths(...)`.

Example:

```rust
#[derive(OpenApi)]
#[openapi(
    paths(
        crate::http::router::health,
        crate::modules::docs::routes::list_sample_pages
    )
)]
pub struct ApiDoc;
```

## How To Document A New Endpoint

For each endpoint:

1. Add request and response DTOs in the module's `dto.rs`.
2. Add `#[derive(ToSchema)]` to DTOs that appear in the API.
3. Add `#[utoipa::path(...)]` above the handler.
4. Add the handler to `paths(...)` in `services/api/src/openapi.rs`.
5. Add any new DTOs to `components(schemas(...))`.
6. Run `cargo check --workspace`.
7. Open `/swagger-ui` and confirm the endpoint appears.

Example response DTO:

```rust
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct PageResponse {
    pub id: String,
    pub title: String,
}
```

Example handler documentation:

```rust
#[utoipa::path(
    get,
    path = "/docs/pages/{page_id}",
    tag = "docs",
    params(
        ("page_id" = String, Path, description = "Document page id")
    ),
    responses(
        (status = 200, description = "Document page", body = PageResponse)
    )
)]
pub async fn get_page() {
    // Handler implementation.
}
```

## Recommended API Documentation Rules

- Keep DTOs in `dto.rs`, not inside `routes.rs`, once they become real product types.
- Use stable operation paths like `/docs/pages/{page_id}`.
- Use module tags such as `docs`, `attendance`, `auth`, and `orgs`.
- Document error responses when handlers start returning real API errors.
- Keep sample/demo endpoints separate from real product endpoints.

## Frontend Usage

The frontend can read the raw OpenAPI document from:

```txt
/api-docs/openapi.json
```

Later this can feed tools such as TypeScript client generation, API contract tests, or Postman imports.

