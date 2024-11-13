//! Load balancer module for distributing requests across backend services.
//!
//! The `LoadBalancer` struct manages a list of backend services and selects one
//! based on a round-robin algorithm. It also supports adding and removing backends dynamically.

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

/// A load balancer that manages backend servers and distributes requests using round-robin.
#[derive(Clone, Debug)]
pub struct LoadBalancer {
    /// List of backend server addresses.
    backends: Arc<Mutex<Vec<SocketAddr>>>,
    /// Index for the next backend in the round-robin sequence.
    current_index: Arc<Mutex<usize>>,
}

impl LoadBalancer {
    /// Creates a new load balancer with an initial set of backends.
    ///
    /// # Parameters
    /// - `backends`: A list of backend server addresses to initialize the load balancer.
    pub fn new(backends: Vec<SocketAddr>) -> Self {
        Self {
            backends: Arc::new(Mutex::new(backends)),
            current_index: Arc::new(Mutex::new(0)),
        }
    }

    /// Selects the next backend using a round-robin strategy.
    ///
    /// # Returns
    /// An optional `SocketAddr` of the selected backend, or `None` if no backends are available.
    pub fn select_backend(&self) -> Option<SocketAddr> {
        let backends = self.backends.lock().unwrap();
        println!("Available backends: {:?}", *backends);

        if backends.is_empty() {
            println!("No backends available");
            return None;
        }

        let mut index = self.current_index.lock().unwrap();
        let backend = backends[*index];
        *index = (*index + 1) % backends.len();
        println!("Selected backend: {}", backend);
        Some(backend)
    }

    /// Adds a backend to the list if it is not already present.
    ///
    /// # Parameters
    /// - `backend`: The address of the backend to add.
    pub fn add_backend(&self, backend: SocketAddr) {
        let mut backends = self.backends.lock().unwrap();
        if !backends.contains(&backend) {
            println!("Adding backend: {}", backend);
            backends.push(backend);
        } else {
            println!("Backend already exists: {}", backend);
        }
    }

    /// Removes a backend from the list.
    ///
    /// # Parameters
    /// - `backend`: The address of the backend to remove.
    pub fn remove_backend(&self, backend: &SocketAddr) {
        let mut backends = self.backends.lock().unwrap();
        backends.retain(|&b| b != *backend);
    }

    /// Returns a list of current backend addresses.
    pub fn get_backends(&self) -> Vec<SocketAddr> {
        let backends = self.backends.lock().unwrap();
        backends.clone()
    }
}
