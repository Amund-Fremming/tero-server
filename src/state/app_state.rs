use std::sync::Arc;

use gustcache::GustCache;
use reqwest::Client;
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use tracing::info;

use crate::{AUTH0_DOMAIN, error::ServerError, quiz::Quiz, spin::Spin};

#[derive(Debug)]
pub struct AppState {
    pool: Pool<Postgres>,
    jwks: Jwks,
    quiz_cache: GustCache<Vec<Quiz>>,
    spin_cache: GustCache<Vec<Spin>>,
    //TODO ADD CACHE FOR SESSIONS
}

#[derive(Debug, Deserialize, Clone)]
pub struct Jwks {
    pub keys: [Jwk; 2],
}

#[allow(dead_code)]
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
        let quiz_cache = GustCache::from_ttl(chrono::Duration::minutes(2));
        let spin_cache = GustCache::from_ttl(chrono::Duration::minutes(2));

        let state = Arc::new(Self {
            pool,
            jwks,
            quiz_cache,
            spin_cache,
        });

        Ok(state)
    }

    pub fn get_pool(&self) -> &Pool<Postgres> {
        &self.pool
    }

    pub fn get_jwks(&self) -> &Jwks {
        &self.jwks
    }

    pub fn get_quiz_cache(&self) -> &GustCache<Vec<Quiz>> {
        &self.quiz_cache
    }

    pub fn get_spin_cache(&self) -> &GustCache<Vec<Spin>> {
        &self.spin_cache
    }
}
