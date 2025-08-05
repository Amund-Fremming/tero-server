use sqlx::{Pool, Postgres, Row, pool, query};
use uuid::Uuid;

use crate::{
    auth::{PutUserRequest, User},
    error::ServerError,
};

pub async fn create_guest_user(pool: &Pool<Postgres>, user: &User) -> Result<Uuid, sqlx::Error> {
    let row = sqlx::query(
        r#"
        INSERT INTO "user" (guest_id, user_type, last_active)
        VALUES ($1, $2, $3)
        RETURNING guest_id;
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

pub async fn get_user_by_auth0_id(
    pool: &Pool<Postgres>,
    auth0_id: String,
) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(r#"SELECT * from "user" WHERE auth0_id = $1"#)
        .bind(&auth0_id)
        .fetch_optional(pool)
        .await
}

pub async fn get_user_by_guest_id(
    pool: &Pool<Postgres>,
    guest_id: Uuid,
) -> Result<Option<User>, sqlx::Error> {
    let mut opt = sqlx::query_as::<_, User>(r#"SELECT * from "user" WHERE guest_id = $1"#)
        .bind(&guest_id)
        .fetch_optional(pool)
        .await?;

    if let Some(ref mut user) = opt {
        user.strip_sensisive_data();
    }

    Ok(opt)
}

pub async fn put_user_by_auth0_id(
    pool: &Pool<Postgres>,
    auth0_id: String,
    put_request: PutUserRequest,
) -> Result<User, sqlx::Error> {
    todo!()
}

pub async fn delete_user_by_auth0_id(
    pool: &Pool<Postgres>,
    auth0_id: String,
) -> Result<(), ServerError> {
    let result = query!(
        r#"
        DELETE FROM "user" WHERE auth0_id = $1;
        "#,
        auth0_id
    )
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(ServerError::Internal("Failed delete".into()));
    }

    Ok(())
}
