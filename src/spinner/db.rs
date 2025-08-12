use sqlx::{Pool, Postgres};

use crate::{common::PagedRequest, error::ServerError, quiz::QuizSession};

pub async fn get_game_by_id(pool: Pool<Postgres>, id: &i32) -> Result<QuizSession, ServerError> {
    todo!()
}

pub async fn get_spinner_page(
    pool: Pool<Postgres>,
    req: &PagedRequest,
) -> Result<QuizSession, ServerError> {
    todo!()
}
