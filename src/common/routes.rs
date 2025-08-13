use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    common::{get_game_session_by_id, typed_search},
    state::AppState,
};

pub fn common_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/search/{game_type}", post(typed_search))
        .route("/get/{game_type}/{game_id}", get(get_game_session_by_id))
        .with_state(state)
}
