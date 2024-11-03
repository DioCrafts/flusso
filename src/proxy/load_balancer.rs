// src/proxy/load_balancer.rs

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

pub struct LoadBalancer {
    backends: Arc<Mutex<Vec<SocketAddr>>>, // Compartido de manera segura y accesible para modificaciones concurrentes
    current_index: Arc<Mutex<usize>>,
}

impl LoadBalancer {
    pub fn new(backends: Vec<SocketAddr>) -> Self {
        Self {
            backends: Arc::new(Mutex::new(backends)),
            current_index: Arc::new(Mutex::new(0)),
        }
    }

    /// Selecciona el siguiente backend de acuerdo al algoritmo Round Robin.
    pub fn select_backend(&self) -> Option<SocketAddr> {
        let backends = self.backends.lock().unwrap();
        if backends.is_empty() {
            return None;
        }

        let mut index = self.current_index.lock().unwrap();
        let backend = backends[*index];
        *index = (*index + 1) % backends.len();
        Some(backend)
    }

    /// Permite añadir un backend en caliente.
    pub fn add_backend(&self, backend: SocketAddr) {
        let mut backends = self.backends.lock().unwrap();
        if !backends.contains(&backend) {
            backends.push(backend);
        }
    }

    /// Permite remover un backend.
    pub fn remove_backend(&self, backend: &SocketAddr) {
        let mut backends = self.backends.lock().unwrap();
        backends.retain(|&b| b != *backend);
    }

    /// Devuelve una lista de los backends actuales.
    pub fn get_backends(&self) -> Vec<SocketAddr> {
        let backends = self.backends.lock().unwrap();
        backends.clone() // Retorna una copia de la lista de backends
    }
}

