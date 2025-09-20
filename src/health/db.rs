use sqlx::{Pool, Postgres};

pub async fn health_check(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT 1").execute(pool).await?;
    Ok(())
}
