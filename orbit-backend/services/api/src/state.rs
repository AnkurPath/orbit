use crate::config::ApiConfig;

#[derive(Clone)]
pub struct AppState {
    pub config: ApiConfig,
}

impl AppState {
    pub fn new(config: ApiConfig) -> Self {
        Self { config }
    }
}
