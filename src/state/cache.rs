use std::{collections::HashMap, sync::RwLock};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    error::ServerError,
    quiz::QuizSession,
    spinner::{Round, Spinner, SpinnerPlayer},
};

pub static ACTIVE_QUIZ_CACHE: Lazy<ActiveQuizGames> = Lazy::new(|| ActiveQuizGames::new());
pub static ACTIVE_SPINNER_CACHE: Lazy<ActiveQuizGames> = Lazy::new(|| ActiveQuizGames::new());

#[derive(Debug, Serialize, Deserialize)]
pub struct ActiveQuizGames {
    games: RwLock<HashMap<Uuid, QuizSession>>,
}

impl ActiveQuizGames {
    pub fn new() -> Self {
        Self {
            games: RwLock::new(HashMap::new()),
        }
    }

    pub fn read<F, R>(&mut self, id: Uuid, read_fn: F) -> Result<R, ServerError>
    where
        F: FnOnce(&QuizSession) -> R,
    {
        let map = self
            .games
            .write()
            .map_err(|_| ServerError::RwLock("Failed to open read lock".into()))?;

        let session = map
            .get(&id)
            .ok_or_else(|| ServerError::GameSession("Quiz".into(), "Does not exist".into()))?;

        Ok(read_fn(session))
    }

    pub fn write<F>(&mut self, id: Uuid, mut write_fn: F) -> Result<(), ServerError>
    where
        F: FnMut(&mut QuizSession),
    {
        let mut map = self.games.write().map_err(|_| {
            ServerError::GameSession("Quiz".into(), "Failed to open write lock".into())
        })?;

        let session = map
            .get_mut(&id)
            .ok_or_else(|| ServerError::GameSession("Quiz".into(), "Does not exist".into()))?;

        write_fn(session);
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpinnerSession {
    game: Spinner,
    rounds: Vec<Round>,
    players: Vec<SpinnerPlayer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActiveSpinnerGames {
    games: RwLock<HashMap<u32, Spinner>>,
}

impl ActiveSpinnerGames {
    pub fn new() -> Self {
        Self {
            games: RwLock::new(HashMap::new()),
        }
    }
}
