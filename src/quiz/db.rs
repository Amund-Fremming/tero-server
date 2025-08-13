use sqlx::{Pool, Postgres, query_as};
use uuid::Uuid;

use crate::{
    common::PagedRequest,
    error::ServerError,
    quiz::{Question, Quiz, QuizSession},
};

pub async fn get_quiz_session_by_id(
    pool: &Pool<Postgres>,
    quiz_id: &Uuid,
) -> Result<QuizSession, ServerError> {
    let quiz = sqlx::query_as::<_, Quiz>(
        r#"
        SELECT id, name, description, "category:GameCategory", iterations, times_played
        FROM quiz
        WHERE id = $1
        "#,
    )
    .bind(quiz_id)
    .fetch_optional(pool)
    .await?
    .ok_or(ServerError::NotFound(format!(
        "Quiz with id {} does not exist",
        quiz_id
    )))?;

    let questions = query_as::<_, Question>(
        r#"
        SELECT id, quiz_id, title
        FROM question
        WHERE quiz_id = $1
        "#,
    )
    .bind(quiz_id)
    .fetch_all(pool)
    .await?;

    let session = QuizSession::from_db(quiz, questions);
    Ok(session)
}

pub async fn get_quiz_page(
    pool: &Pool<Postgres>,
    req: &PagedRequest,
) -> Result<QuizSession, ServerError> {
    todo!();
}
