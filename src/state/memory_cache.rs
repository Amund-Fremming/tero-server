use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
};

use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::{error::ServerError, quiz::Quiz, spinner::Spinner};

/*
    TODO
        - expand new fn to take in option for time
        - invalidate cache fn
        - maybe a fn for insert, get, invalidate sinlge entry?
        - split this into own repo
        - rename memory cache
*/

pub static QUIZ_PAGE_CACHE: Lazy<MemoryCache<Vec<Quiz>>> =
    Lazy::new(|| MemoryCache::new(chrono::Duration::minutes(5)));
pub static SPINNER_PAGE_CACHE: Lazy<MemoryCache<Vec<Spinner>>> =
    Lazy::new(|| MemoryCache::new(chrono::Duration::minutes(5)));

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CacheEntry<T: Clone> {
    pub timestamp: DateTime<Utc>,
    data: T,
}

#[derive(Debug)]
pub struct MemoryCache<T: Clone> {
    cache: RwLock<HashMap<u64, CacheEntry<T>>>,
    ttl: chrono::TimeDelta,
}

impl<T: Clone> MemoryCache<T> {
    pub fn new(ttl: chrono::TimeDelta) -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
            ttl,
        }
    }

    fn generate_hash<TKey>(&self, value: &TKey) -> u64
    where
        TKey: Hash,
    {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }

    pub async fn get<F, TKey>(&self, req: &TKey, db_fn: F) -> Result<T, ServerError>
    where
        F: AsyncFnOnce() -> Result<T, ServerError>,
        TKey: Hash,
    {
        let key = self.generate_hash(req);
        let mut map = self.cache.write().await;

        if let Some(entry) = map.get_mut(&key) {
            if entry.timestamp + self.ttl > Utc::now() {
                entry.timestamp = Utc::now();
            }
        };

        // Release lock while db operation finishes
        drop(map);

        let data = db_fn().await?;
        let cache_entry = CacheEntry {
            data: data.clone(),
            timestamp: Utc::now(),
        };

        let mut map = self.cache.write().await;
        map.insert(key, cache_entry.clone());

        Ok(data)
    }
}
