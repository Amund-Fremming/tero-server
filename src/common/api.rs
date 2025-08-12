use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use reqwest::StatusCode;

use crate::{
    common::{GameType, PagedRequest},
    error::ServerError,
    state::AppState,
};

pub async fn typed_search(
    State(state): State<AppState>,
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
    };
}
