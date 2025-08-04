use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Sqlx failed: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Api error: {1}")]
    Api(StatusCode, String),

    #[error("Permission error: {0}")]
    Permission(String),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ServerError::Sqlx(e) => {
                error!("Sqlx failed with error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, String::new())
            }
            ServerError::Internal(e) => {
                error!("Internal server error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, e)
            }
            ServerError::Api(sc, msg) => {
                error!("Api error: {} - {}", sc, msg);
                (sc, msg)
            }
            ServerError::Permission(e) => {
                error!("Missing permission: {}", e);
                (StatusCode::FORBIDDEN, e)
            }
        }
        .into_response()
    }
}
