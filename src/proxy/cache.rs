// Cache de respuestas (opcional)
// src/proxy/cache.rs

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct CacheEntry {
    pub response: Vec<u8>,        // Almacena la respuesta en bytes
    pub timestamp: Instant,       // Marca el tiempo de almacenamiento
}

#[derive(Clone)]
pub struct Cache {
    data: Arc<Mutex<HashMap<String, CacheEntry>>>,
    ttl: Duration,
}

impl Cache {
    pub fn new(ttl_seconds: u64) -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
            ttl: Duration::from_secs(ttl_seconds),
        }
    }

    /// Almacena una respuesta en la caché.
    pub fn store(&self, key: String, response: Vec<u8>) {
        let entry = CacheEntry {
            response,
            timestamp: Instant::now(),
        };
        let mut data = self.data.lock().unwrap();
        data.insert(key, entry);
    }

    /// Recupera una respuesta de la caché, si es válida (no expirada).
    pub fn retrieve(&self, key: &str) -> Option<Vec<u8>> {
        let mut data = self.data.lock().unwrap();
        
        if let Some(entry) = data.get(key) {
            if entry.timestamp.elapsed() < self.ttl {
                return Some(entry.response.clone());
            } else {
                // Remueve la entrada si ha expirado
                data.remove(key);
            }
        }
        None
    }

    /// Limpia las entradas expiradas de la caché.
    pub fn clean_expired(&self) {
        let mut data = self.data.lock().unwrap();
        let ttl = self.ttl;

        data.retain(|_, entry| entry.timestamp.elapsed() < ttl);
    }
}
