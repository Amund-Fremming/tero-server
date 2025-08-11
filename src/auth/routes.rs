use std::sync::Arc;

use axum::{
    Router,
    routing::{get, patch, post, put},
};

use crate::{
    auth::{
        auth0_trigger_endpoint, create_guest_user, delete_user, get_user_from_subject,
        list_all_users, patch_user, patch_user_activity,
    },
    state::AppState,
};

pub fn public_auth_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", post(create_guest_user))
        .with_state(state.clone())
}

pub fn protected_auth_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(get_user_from_subject))
        .route(
            "/{user_id}",
            patch(patch_user)
                .delete(delete_user)
                .post(auth0_trigger_endpoint),
        )
        .route("/list", get(list_all_users))
        .route("/activity/{user_id}", put(patch_user_activity))
        .with_state(state.clone())
}
