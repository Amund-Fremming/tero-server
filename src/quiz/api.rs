use axum::{Extension, http::StatusCode, response::IntoResponse};
use tracing::info;

use crate::{auth::Subject, error::ServerError};

pub async fn dummy_quiz(
    Extension(subject): Extension<Subject>,
) -> Result<impl IntoResponse, ServerError> {
    info!("Recieved subject: {:?}", subject);

    Ok((StatusCode::OK, String::from("HEY")))
}
