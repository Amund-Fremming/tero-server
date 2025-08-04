use std::sync::Arc;

use axum::{Router, routing::post};

use crate::{auth::create_guest_user, state::AppState};

pub fn auth_routes(state: Arc<AppState>) -> Router {
    Router::new().route(
        "/guest-user",
        post(create_guest_user).with_state(state.clone()),
    )
}
