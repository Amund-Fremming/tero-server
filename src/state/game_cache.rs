use std::{collections::HashMap, sync::RwLock};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{error::ServerError, quiz::QuizSession, spinner::SpinnerSession};

pub static ACTIVE_QUIZ_CACHE: Lazy<GameCache<QuizSession>> = Lazy::new(|| GameCache::new());
pub static ACTIVE_SPINNER_CACHE: Lazy<GameCache<SpinnerSession>> = Lazy::new(|| GameCache::new());

#[derive(Debug, Serialize, Deserialize)]
pub struct GameCache<T> {
    pub games: RwLock<HashMap<Uuid, T>>,
}

impl<T> GameCache<T> {
    pub fn new() -> Self {
        Self {
            games: RwLock::new(HashMap::new()),
        }
    }

    pub fn read<F, R>(&self, id: &Uuid, read_fn: F) -> Result<R, ServerError>
    where
        F: FnOnce(&T) -> R,
    {
        let map = self
            .games
            .write()
            .map_err(|_| ServerError::RwLock("Failed to open read lock on game cache".into()))?;

        let session = map
            .get(&id)
            .ok_or_else(|| ServerError::GameSession("Quiz".into(), "Does not exist".into()))?;

        Ok(read_fn(session))
    }

    pub fn write<F>(&self, id: &Uuid, mut write_fn: F) -> Result<(), ServerError>
    where
        F: FnMut(&mut T),
    {
        let mut map = self.games.write().map_err(|_| {
            ServerError::GameSession(
                "Quiz".into(),
                "Failed to open write lock on game cache".into(),
            )
        })?;

        let session = map
            .get_mut(&id)
            .ok_or_else(|| ServerError::GameSession("Quiz".into(), "Does not exist".into()))?;

        write_fn(session);

        Ok(())
    }
}
