use std::sync::Arc;

use axum::{Router, middleware::from_fn, routing::get};

use crate::{mw::subject_mw, quiz::dummy_quiz, state::AppState};

pub fn quiz_routes(state: Arc<AppState>) -> Router {
    Router::new().route("/", get(dummy_quiz))
}
