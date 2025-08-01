use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Sqlx failed: {0}")]
    SqlxError(#[from] sqlx::Error),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ServerError::SqlxError(e) => {
                error!("{:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, String::new())
            }
        }
        .into_response()
    }
}
