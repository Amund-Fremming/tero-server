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

    #[error("Permission error")]
    Permission(Vec<String>),

    #[error("Access denied error")]
    AccessDenied,

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),

    #[error("JWT verification error: {0}")]
    JwtVerification(String),

    #[error("Json error: {0}")]
    Json(String),
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
                (StatusCode::INTERNAL_SERVER_ERROR, String::new())
            }
            ServerError::Api(sc, msg) => {
                error!("Api error: {} - {}", sc, msg);
                (sc, msg)
            }
            ServerError::Permission(missing) => {
                error!("Missing permission(s): {:?}", missing);
                (StatusCode::FORBIDDEN, String::from("Missing permission(s)"))
            }
            ServerError::NotFound(e) => {
                error!("Entity not found: {}", e);
                (StatusCode::NOT_FOUND, e)
            }
            ServerError::AccessDenied => {
                error!("Access denied for requesting entity");
                (StatusCode::FORBIDDEN, String::from("Access denied"))
            }
            ServerError::Request(e) => {
                error!("Failed to send request: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    String::from("Failed to access third party"),
                )
            }
            ServerError::JwtVerification(e) => {
                error!("Failed to verify JWT: {}", e);
                (StatusCode::UNAUTHORIZED, String::new())
            }
            ServerError::Json(e) => {
                error!("Json error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, String::new())
            }
        }
        .into_response()
    }
}
