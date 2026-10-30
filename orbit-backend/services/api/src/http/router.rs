use axum::{routing::get, Router};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{modules, openapi::ApiDoc, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health))
        .nest("/auth", modules::auth::routes())
        .nest("/orgs", modules::orgs::routes())
        .nest("/users", modules::users::routes())
        .nest("/docs", modules::docs::routes())
        .nest("/attendance", modules::attendance::routes())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
}

#[utoipa::path(
    get,
    path = "/health",
    tag = "health",
    responses(
        (status = 200, description = "API is healthy", body = String)
    )
)]
pub(crate) async fn health() -> &'static str {
    "ok"
}
