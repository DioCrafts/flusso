//! Cache module for storing and retrieving HTTP responses.
//!
//! The `Cache` struct provides a time-limited storage of responses using a
//! specified time-to-live (TTL) duration.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Represents an entry in the cache with response data and a timestamp.
#[derive(Clone)]
pub struct CacheEntry {
    pub response: Vec<u8>,       // Stores response in bytes
    pub timestamp: Instant,      // Marks the storage time
}

/// Cache for HTTP responses with a time-to-live (TTL) for entries.
#[derive(Clone)]
pub struct Cache {
    data: Arc<Mutex<HashMap<String, CacheEntry>>>,
    ttl: Duration,
}

impl Cache {
    /// Creates a new cache with the specified TTL (in seconds).
    pub fn new(ttl_seconds: u64) -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
            ttl: Duration::from_secs(ttl_seconds),
        }
    }

    /// Stores a response in the cache with a unique key.
    pub fn store(&self, key: String, response: Vec<u8>) {
        let entry = CacheEntry {
            response,
            timestamp: Instant::now(),
        };
        let mut data = self.data.lock().unwrap();
        data.insert(key, entry);
    }

    /// Retrieves a response from the cache if it is still valid.
    pub fn retrieve(&self, key: &str) -> Option<Vec<u8>> {
        let mut data = self.data.lock().unwrap();
        if let Some(entry) = data.get(key) {
            if entry.timestamp.elapsed() < self.ttl {
                return Some(entry.response.clone());
            } else {
                data.remove(key);
            }
        }
        None
    }

    /// Cleans expired entries from the cache.
    pub fn clean_expired(&self) {
        let mut data = self.data.lock().unwrap();
        let ttl = self.ttl;
        data.retain(|_, entry| entry.timestamp.elapsed() < ttl);
    }
}
