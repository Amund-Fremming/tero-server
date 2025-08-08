use std::sync::Arc;

use axum::{
    Router,
    middleware::from_fn,
    routing::{get, post, put},
};

use crate::{
    auth::{
        auth0_trigger_endpoint, create_guest_user, delete_user, get_user_from_subject, put_user,
        update_user_activity,
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
        .route(
            "/",
            get(get_user_from_subject)
                .put(put_user)
                .delete(delete_user)
                .post(auth0_trigger_endpoint),
        )
        .route("/{user_id}", put(update_user_activity))
        .with_state(state.clone())
}
