use axum::{
    Router,
    response::{IntoResponse, Response},
    routing::get,
};

use crate::error::ServerError;

pub fn health_routes() -> Router {
    Router::new()
        .route("/", get(health))
        .route("/detailed", get(health_detailed))
}

async fn health() -> impl IntoResponse {
    "OK".into_response()
}

async fn health_detailed() -> Result<Response, ServerError> {
    todo!()
}
