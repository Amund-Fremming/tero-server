use std::sync::Arc;

use axum::{
    Router,
    extract::State,
    response::IntoResponse,
    routing::{get, post},
};

use crate::common::{app_state::AppState, server_error::ServerError};

pub fn spingame_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(initiate_game).post(create_game))
        .route("/persist", post(persist_gamesession))
        .with_state(state.clone())
}

async fn initiate_game(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ServerError> {
    todo!();
    Ok(())
}

async fn create_game(State(state): State<Arc<AppState>>) -> Result<impl IntoResponse, ServerError> {
    todo!();
    Ok(())
}
