use std::sync::Arc;

use axum::{
    Router,
    middleware::from_fn,
    routing::{get, post},
};

use crate::{
    auth::{create_guest_user, get_user_from_subject},
    mw::subject_mw,
    state::AppState,
};

pub fn public_auth_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", post(create_guest_user))
        .with_state(state.clone())
}

pub fn protected_auth_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(get_user_from_subject).post(create_guest_user))
        .with_state(state.clone())
        .layer(from_fn(subject_mw))
}
