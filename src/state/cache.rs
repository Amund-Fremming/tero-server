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
            .map_err(|_| ServerError::RwLock("Failed to open read lock".into()))?;

        let session = map
            .get(&id)
            .ok_or_else(|| ServerError::GameSession("Quiz".into(), "Does not exist".into()))?;

        Ok(read_fn(session))
    }

    pub fn write<F>(&mut self, id: &Uuid, mut write_fn: F) -> Result<(), ServerError>
    where
        F: FnMut(&mut T),
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
pub struct ListPageCache<T> {
    page: RwLock<Vec<T>>,
}

impl<T> ListPageCache<T> {
    pub fn new() -> Self {
        Self {
            page: RwLock::new(Vec::new()),
        }
    }

    /*
    Not in cache
        hit db
        insert into cache
        return data

    In cache
        reset timeout
        return data

     */

    /*
    Page needs
        incremental browsing (handled)
        filtered by category
        filtered by most played
        search

    Sol 1
        map paged_request to the page result
        handles different complex queries
        needs to inject new entries into cache on creation
        could also be valid for searching if query in in paged_request
        generate a hash key for the object incomming

    Sol 2
        Only cache incremental browsing
        no cache for complex filtering
        only refreshes cache if page is not in cache or page is under 20 items

     */
}
