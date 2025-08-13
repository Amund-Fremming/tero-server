use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use reqwest::StatusCode;
use uuid::Uuid;

use crate::{
    common::{GameType, PagedRequest},
    error::ServerError,
    quiz::{get_quiz_page, get_quiz_session_by_id},
    spinner::{get_spinner_page, get_spinner_session_by_id},
    state::AppState,
};

pub async fn typed_search(
    State(state): State<Arc<AppState>>,
    Path(game_type): Path<GameType>,
    Json(request): Json<PagedRequest>,
) -> Result<impl IntoResponse, ServerError> {
    match game_type {
        GameType::Quiz => {
            let page = get_quiz_page(state.get_pool(), &request).await?;
            Ok((StatusCode::OK, Json(page)))
        }
        GameType::Spinner => {
            let page = get_spinner_page(state.get_pool(), &request).await?;
            Ok((StatusCode::OK, Json(page)))
        }
    }
}

pub async fn get_game_session_by_id(
    State(state): State<Arc<AppState>>,
    Path((game_type, game_id)): Path<(GameType, Uuid)>,
) -> Result<impl IntoResponse, ServerError> {
    match game_type {
        GameType::Quiz => {
            let game = get_quiz_session_by_id(state.get_pool(), &game_id).await?;
            Ok((StatusCode::OK, Json(game)))
        }
        GameType::Spinner => {
            let game = get_spinner_session_by_id(state.get_pool(), &game_id).await?;
            Ok((StatusCode::OK, Json(game)))
        }
    }
}
