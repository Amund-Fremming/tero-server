use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
    sync::RwLock,
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{common::PagedRequest, error::ServerError};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct CacheEntry<T: Clone> {
    pub timestamp: DateTime<Utc>,
    page: [T; 20],
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

    pub fn try_get(&self, req: &PagedRequest) -> Result<Option<CacheEntry<T>>, ServerError> {
        let key = req.generate_hash();

        let map = self
            .cache
            .read()
            .map_err(|_| ServerError::RwLock("Failed to open read lock on page cache".into()))?;

        let entry = match map.get(&key).cloned() {
            Some(entry) => entry,
            None => return Ok(None),
        };

        let offset = chrono::Duration::minutes(10);
        if entry.timestamp + offset < Utc::now() {
            return Ok(None);
        };

        self.update_timestamp(&key)?;
        Ok(Some(entry))
    }

    fn update_timestamp(&self, key: &u64) -> Result<(), ServerError> {
        let mut map = self
            .cache
            .write()
            .map_err(|_| ServerError::RwLock("Failed to open write lock on page cache".into()))?;

        if let Some(entry) = map.get_mut(key) {
            entry.timestamp = Utc::now();
        };

        Ok(())
    }

    pub fn insert(&self, req: &PagedRequest, page: [T; 20]) -> Result<(), ServerError> {
        let key = req.generate_hash();

        let mut map = self
            .cache
            .write()
            .map_err(|_| ServerError::RwLock("Failed to open write lock on page cache".into()))?;

        let cache_entry = CacheEntry {
            timestamp: Utc::now(),
            page,
        };

        map.insert(key, cache_entry);
        Ok(())
    }

    /*
    Cache
        hash_key -> (arr of 20 T, timestamp)

    Not in cache
        hit db
        insert into cache
        return data

    In cache
        reset timeout
        return data
        */
}
