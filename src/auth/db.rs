use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;

use crate::{auth::User, error::ServerError};

pub async fn create_guest_user(pool: &Pool<Postgres>, user: &User) -> Result<Uuid, ServerError> {
    let row = sqlx::query(
        r#"
        INSERT INTO "user" ("guest_id", "user_type", "last_active")
        VALUES ($1, $2, $3)
        RETURNING "guest_id";
    "#,
    )
    .bind(&user.guest_id)
    .bind(&user.user_type)
    .bind(&user.last_active)
    .fetch_one(pool)
    .await?;

    let guest_id = row.get("guest_id");

    Ok(guest_id)
}
