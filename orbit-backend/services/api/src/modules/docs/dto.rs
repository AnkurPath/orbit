use serde::Serialize;
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct SamplePageResponse {
    pub id: String,
    pub workspace_id: String,
    pub title: String,
    pub blocks: Vec<SampleBlock>,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct SampleBlock {
    pub id: String,
    pub block_type: SampleBlockType,
    pub text: String,
    pub position: i32,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum SampleBlockType {
    Heading,
    Paragraph,
    Todo,
}
