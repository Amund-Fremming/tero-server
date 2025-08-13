use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    response::{IntoResponse, Response},
};
use reqwest::StatusCode;
use uuid::Uuid;

use crate::{
    common::{GameApiWrapper, GameType, PagedRequest, PagedResponse},
    error::ServerError,
    quiz::{get_quiz_page, get_quiz_session_by_id},
    spinner::{get_spinner_page, get_spinner_session_by_id},
    state::{AppState, QUIZ_PAGE_CACHE, SPINNER_PAGE_CACHE},
};

#[axum::debug_handler]
pub async fn typed_search(
    State(state): State<Arc<AppState>>,
    Path(game_type): Path<GameType>,
    Json(request): Json<PagedRequest>,
) -> Result<Response, ServerError> {
    let response = match game_type {
        GameType::Quiz => {
            let quizzes = QUIZ_PAGE_CACHE
                .get(&request, || get_quiz_page(state.get_pool(), &request))
                .await?;

            PagedResponse::from_quizzes(quizzes)
        }
        GameType::Spinner => {
            let spinners = SPINNER_PAGE_CACHE
                .get(&request, || get_spinner_page(state.get_pool(), &request))
                .await?;

            PagedResponse::from_spinners(spinners)
        }
    };

    Ok((StatusCode::OK, Json(response)).into_response())
}

pub async fn get_game_session_by_id(
    State(state): State<Arc<AppState>>,
    Path((game_type, game_id)): Path<(GameType, Uuid)>,
) -> Result<impl IntoResponse, ServerError> {
    let wrapper = match game_type {
        GameType::Quiz => {
            let game = get_quiz_session_by_id(state.get_pool(), &game_id).await?;
            GameApiWrapper::Quiz(game)
        }
        GameType::Spinner => {
            let game = get_spinner_session_by_id(state.get_pool(), &game_id).await?;
            GameApiWrapper::Spinner(game)
        }
    };

    Ok((StatusCode::OK, Json(wrapper)))
}
