use std::{collections::HashMap, sync::RwLock};

use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::{common::PagedRequest, error::ServerError, quiz::Quiz, spinner::Spinner};

pub static QUIZ_PAGE_CACHE: Lazy<PagedCache<Quiz>> = Lazy::new(|| PagedCache::new());
pub static SPINNER_PAGE_CACHE: Lazy<PagedCache<Spinner>> = Lazy::new(|| PagedCache::new());

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CacheEntry<T: Clone> {
    pub timestamp: DateTime<Utc>,
    page: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PagedCache<T: Clone> {
    cache: RwLock<HashMap<u64, CacheEntry<T>>>,
}

impl<T: Clone> PagedCache<T> {
    pub fn new() -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
        }
    }

    pub async fn get<F>(&self, req: &PagedRequest, db_fn: F) -> Result<Vec<T>, ServerError>
    where
        F: AsyncFnOnce() -> Result<Vec<T>, ServerError>,
    {
        let key = req.generate_hash();

        let mut map = self
            .cache
            .write()
            .map_err(|_| ServerError::RwLock("Failed to open read lock on page cache".into()))?;

        let offset = chrono::Duration::minutes(10);
        if let Some(entry) = map.get_mut(&key) {
            if entry.timestamp + offset > Utc::now() {
                entry.timestamp = Utc::now();
            }
        };

        // Release lock while db operation finishes
        drop(map);

        let entries = db_fn().await?;
        let entry = CacheEntry {
            page: entries,
            timestamp: Utc::now(),
        };

        let mut map = self
            .cache
            .write()
            .map_err(|_| ServerError::RwLock("Failed to open read lock on page cache".into()))?;

        map.insert(key, entry.clone());

        Ok(entry.page)
    }
}
