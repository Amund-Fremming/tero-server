use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
    common::{models::PagedRequest, server_error::ServerError},
    spin::models::{Round, Spin, SpinSession},
};

pub async fn get_spin_session_by_id(
    pool: &Pool<Postgres>,
    spinner_id: &Uuid,
) -> Result<SpinSession, ServerError> {
    let spinner = sqlx::query_as::<_, Spin>(
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

    let session = SpinSession::from_db(spinner, rounds);

    Ok(session)
}

pub async fn get_spin_page(
    pool: &Pool<Postgres>,
    req: &PagedRequest,
) -> Result<Vec<Spin>, ServerError> {
    let mut sql = String::from(
        r#"
        SELECT id, host_id, name, description, category, iterations, times_played
        FROM spinner
        "#,
    );

    let mut query = Vec::new();
    let offset = 20 * req.page_num;
    let limit = 20;

    if let Some(category) = &req.category {
        query.push(format!(" category = '{}'", category.as_str()));
    }

    query.push(format!("LIMIT {} OFFSET {}", limit, offset));
    sql.push_str(format!("WHERE {}", query.join(" AND ")).as_str());
    let spinners = sqlx::query_as::<_, Spin>(&sql).fetch_all(pool).await?;

    Ok(spinners)
}
