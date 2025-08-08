use std::sync::Arc;

use reqwest::Client;
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use tracing::info;

use crate::{AUTH0_DOMAIN, error::ServerError};

#[derive(Debug, Clone)]
pub struct AppState {
    pool: Pool<Postgres>,
    jwks: Jwks,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Jwks {
    pub keys: [Jwk; 2],
}

#[derive(Debug, Deserialize, Clone)]
pub struct Jwk {
    pub kid: String,
    pub n: String,
    pub e: String,
    pub kty: String,
    pub alg: String,
    #[serde(rename(deserialize = "use"))]
    pub use_: String,
}

impl AppState {
    pub async fn from_connection_string(s: &str) -> Result<Arc<Self>, ServerError> {
        let pool = Pool::<Postgres>::connect(&s).await?;

        let url = format!("{}.well-known/jwks.json", *AUTH0_DOMAIN);
        let response = Client::new().get(url).send().await?;
        info!("JWKs Response: {}", response.status());
        let jwks = response.json::<Jwks>().await?;

        let state = Arc::new(Self { pool, jwks });

        Ok(state)
    }

    pub fn get_pool(&self) -> &Pool<Postgres> {
        &self.pool
    }

    pub fn get_jwks(&self) -> &Jwks {
        &self.jwks
    }
}
