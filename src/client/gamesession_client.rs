use core::fmt;

use reqwest::Client;

#[derive(Debug, thiserror::Error)]
pub enum GameSessionClientError {
    #[error("Failed to initialize game: {0}")]
    Initialize(String),

    #[error("Failed to initialize game: {0}")]
    Create(String),

    #[error("Http request failed: {0}")]
    Http(#[from] reqwest::Error),
}

pub enum Hub {
    Quiz,
    Spin,
}

impl fmt::Display for Hub {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Hub::Quiz => write!(f, "quiz"),
            Hub::Spin => write!(f, "spin"),
        }
    }
}

pub struct GameSessionClient {
    domain: String,
}

impl GameSessionClient {
    pub fn new(domain: impl Into<String>) -> Self {
        let domain = domain.into();

        Self { domain }
    }

    async fn initiate_game(&self, client: &Client) -> Result<(), GameSessionClientError> {
        //
    }

    async fn create_game(&self, client: &Client) -> Result<(), GameSessionClientError> {
        //
    }
}
