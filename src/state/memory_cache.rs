use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
};

use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::{error::ServerError, quiz::Quiz, spinner::Spinner};

// Move out maybe?, create a func that returns these lazy functions
pub static QUIZ_PAGE_CACHE: Lazy<MemoryCache<Quiz>> = Lazy::new(|| MemoryCache::new());
pub static SPINNER_PAGE_CACHE: Lazy<MemoryCache<Spinner>> = Lazy::new(|| MemoryCache::new());

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CacheEntry<T: Clone> {
    pub timestamp: DateTime<Utc>,
    data: T,
}

#[derive(Debug)]
pub struct MemoryCache<T: Clone> {
    cache: RwLock<HashMap<u64, CacheEntry<T>>>,
}

impl<T: Clone> MemoryCache<T> {
    pub fn new() -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
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

        let offset = chrono::Duration::minutes(10);
        if let Some(entry) = map.get_mut(&key) {
            if entry.timestamp + offset > Utc::now() {
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
