# Orbit Backend

Rust backend workspace for the Orbit SaaS product.

The current shape is a service-ready modular monolith:

- `services/api` is the Axum HTTP API.
- `services/worker` runs background jobs.
- `crates/*` holds shared Rust libraries used by deployable services.
- `migrations` owns database migrations.

Read:

- [docs/backend-structure.md](docs/backend-structure.md) for the folder-by-folder explanation
- [docs/api-docs.md](docs/api-docs.md) for OpenAPI and Swagger UI usage

## Local Dependencies

Start Postgres, Redis, MinIO, and Mailpit:

```sh
docker compose up -d
```

Copy the sample environment file before running services locally:

```sh
cp .env.example .env
```

Run the API:

```sh
cargo run -p orbit-api
```

Useful local URLs:

- API health: `http://127.0.0.1:8080/health`
- Swagger UI: `http://127.0.0.1:8080/swagger-ui`
- OpenAPI JSON: `http://127.0.0.1:8080/api-docs/openapi.json`
- Sample docs API: `http://127.0.0.1:8080/docs/sample-pages`
- MinIO console: `http://127.0.0.1:9001`
- Mailpit UI: `http://127.0.0.1:8025`

