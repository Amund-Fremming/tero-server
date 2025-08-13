use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
    common::PagedRequest,
    error::ServerError,
    quiz::QuizSession,
    spinner::{Round, Spinner, SpinnerSession},
};

pub async fn get_spinner_session_by_id(
    pool: &Pool<Postgres>,
    spinner_id: &Uuid,
) -> Result<SpinnerSession, ServerError> {
    let spinner = sqlx::query_as::<_, Spinner>(
        r#"
        SELECT id, host_id, name, description, category, iterations, times_played
        FROM spinner
        WHERE id = $1
        "#,
    )
    .bind(spinner_id)
    .fetch_optional(pool)
    .await?
    .ok_or(ServerError::NotFound(format!(
        "Spinner with id {} was not found",
        spinner_id
    )))?;

    let rounds = sqlx::query_as::<_, Round>(
        r#"
        SELECT id, spinner_id, participants, read_before, title
        FROM round
        WHERE spinner_id = $1 
        "#,
    )
    .bind(spinner_id)
    .fetch_all(pool)
    .await?;

    let session = SpinnerSession::from_db(spinner, rounds);

    Ok(session)
}

pub async fn get_spinner_page(
    pool: &Pool<Postgres>,
    req: &PagedRequest,
) -> Result<QuizSession, ServerError> {
    todo!()
}
