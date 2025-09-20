use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    response::{IntoResponse, Response},
    routing::post,
};
use reqwest::StatusCode;

use crate::{
    common::{
        app_state::AppState,
        models::{GameSessionRequest, GameType, PagedRequest, PagedResponse},
        server_error::ServerError,
    },
    quiz::{
        db::{get_quiz_page, tx_persist_quizsession},
        models::QuizSession,
    },
    spin::{
        db::{get_spin_page, tx_persist_spinsession},
        models::SpinSession,
    },
};

pub fn common_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/session/persist", post(persist_gamesession))
        .route("/search/{game_type}", post(typed_search))
        .with_state(state)
}

async fn persist_gamesession(
    State(state): State<Arc<AppState>>,
    Json(request): Json<GameSessionRequest>,
) -> Result<impl IntoResponse, ServerError> {
    let mut tx = state.get_pool().begin().await?;

    match request.game_type {
        GameType::Spin => {
            let gamesession: SpinSession = serde_json::from_value(request.payload)?;
            tx_persist_spinsession(&mut tx, &gamesession).await?;
        }
        GameType::Quiz => {
            let gamesession: QuizSession = serde_json::from_value(request.payload)?;
            tx_persist_quizsession(&mut tx, &gamesession).await?;
        }
    }

    Ok(())
}

async fn typed_search(
    State(state): State<Arc<AppState>>,
    Path(game_type): Path<GameType>,
    Json(request): Json<PagedRequest>,
) -> Result<Response, ServerError> {
    let response = match game_type {
        GameType::Quiz => {
            let quizzes = state
                .get_quiz_cache()
                .get(&request, || get_quiz_page(state.get_pool(), &request))
                .await?;

            PagedResponse::from_quizzes(quizzes)
        }
        GameType::Spin => {
            let spinners = state
                .get_spin_cache()
                .get(&request, || get_spin_page(state.get_pool(), &request))
                .await?;

            PagedResponse::from_spinners(spinners)
        }
    };

    Ok((StatusCode::OK, Json(response)).into_response())
}
