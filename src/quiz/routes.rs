use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{quiz::dummy_quiz, state::AppState};

pub fn quiz_routes(state: Arc<AppState>) -> Router {
    Router::new().route("/", get(dummy_quiz))
}
