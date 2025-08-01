use std::sync::Arc;

use sqlx::{Pool, Postgres};

use crate::error::ServerError;

#[derive(Debug, Clone)]
pub struct AppState {
    pool: Pool<Postgres>,
}

impl AppState {
    pub async fn from_connection_string(s: &str) -> Result<Arc<Self>, ServerError> {
        let pool = Pool::<Postgres>::connect(&s).await?;
        let state = Arc::new(Self { pool });

        Ok(state)
    }

    pub fn get_pool(&self) -> &Pool<Postgres> {
        &self.pool
    }
}
