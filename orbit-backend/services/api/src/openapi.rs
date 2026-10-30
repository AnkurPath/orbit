use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Orbit API",
        version = "0.1.0",
        description = "Backend API for the Orbit workspace platform."
    ),
    paths(
        crate::http::router::health,
        crate::modules::docs::routes::health,
        crate::modules::docs::routes::list_sample_pages,
        crate::modules::docs::routes::get_sample_page
    ),
    components(
        schemas(
            crate::modules::docs::dto::SampleBlock,
            crate::modules::docs::dto::SampleBlockType,
            crate::modules::docs::dto::SamplePageResponse
        )
    ),
    tags(
        (name = "health", description = "Service health checks"),
        (name = "docs", description = "Notion-like document APIs")
    )
)]
pub struct ApiDoc;
