use axum::{extract::Path, routing::get, Json, Router};

use crate::{modules::docs::dto::SamplePageResponse, state::AppState};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(health))
        .route("/sample-pages", get(list_sample_pages))
        .route("/sample-pages/{page_id}", get(get_sample_page))
}

#[utoipa::path(
    get,
    path = "/docs/health",
    tag = "docs",
    responses(
        (status = 200, description = "Docs module is healthy", body = String)
    )
)]
pub(crate) async fn health() -> &'static str {
    "docs ok"
}

#[utoipa::path(
    get,
    path = "/docs/sample-pages",
    tag = "docs",
    responses(
        (status = 200, description = "Sample document pages", body = [SamplePageResponse])
    )
)]
pub(crate) async fn list_sample_pages() -> Json<Vec<SamplePageResponse>> {
    Json(vec![sample_page("page_demo_project_plan")])
}

#[utoipa::path(
    get,
    path = "/docs/sample-pages/{page_id}",
    tag = "docs",
    params(
        ("page_id" = String, Path, description = "Document page id")
    ),
    responses(
        (status = 200, description = "Sample document page", body = SamplePageResponse)
    )
)]
pub(crate) async fn get_sample_page(Path(page_id): Path<String>) -> Json<SamplePageResponse> {
    Json(sample_page(&page_id))
}

fn sample_page(page_id: &str) -> SamplePageResponse {
    use crate::modules::docs::dto::{SampleBlock, SampleBlockType};

    SamplePageResponse {
        id: page_id.to_owned(),
        workspace_id: "workspace_orbit_demo".to_owned(),
        title: "Orbit v1 Product Plan".to_owned(),
        blocks: vec![
            SampleBlock {
                id: "block_heading_intro".to_owned(),
                block_type: SampleBlockType::Heading,
                text: "Build the docs and attendance foundation".to_owned(),
                position: 1,
            },
            SampleBlock {
                id: "block_paragraph_scope".to_owned(),
                block_type: SampleBlockType::Paragraph,
                text: "Version one focuses on Notion-like pages, block editing, and attendance tracking.".to_owned(),
                position: 2,
            },
            SampleBlock {
                id: "block_todo_schema".to_owned(),
                block_type: SampleBlockType::Todo,
                text: "Design docs_pages and docs_blocks tables.".to_owned(),
                position: 3,
            },
        ],
    }
}
