use axum::Router;

use crate::{http, state::AppState};

pub fn build(state: AppState) -> Router {
    http::router::router().with_state(state)
}
