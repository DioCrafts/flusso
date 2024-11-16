//! Router module for managing request routing to backend servers.
//!
//! The `Router` struct stores route mappings, associating paths with backend servers.

use std::collections::HashMap;
use std::net::SocketAddr;

/// Represents a route with a path and its associated backend server address.
#[derive(Clone, Debug)]
pub struct Route {
    pub path: String,
    pub backend: SocketAddr,
}

/// A router that maps paths to backend servers.
#[derive(Default)]
pub struct Router {
    routes: HashMap<String, Route>,
}

impl Router {
    /// Creates a new, empty router.
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    /// Adds a route with a specified path and backend server address.
    pub fn add_route(&mut self, path: String, backend: SocketAddr) {
        let route = Route { path: path.clone(), backend };
        self.routes.insert(path, route);
    }

    /// Retrieves the backend address for a given request path.
    pub fn get_backend(&self, request_path: &str) -> Option<SocketAddr> {
        for (path, route) in &self.routes {
            if request_path.starts_with(path) {
                return Some(route.backend);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_retrieve_route() {
        let mut router = Router::new();
        let backend_addr: SocketAddr = "127.0.0.1:8081".parse().unwrap();

        router.add_route("/api".to_string(), backend_addr);

        assert_eq!(router.get_backend("/api/v1"), Some(backend_addr));
        assert_eq!(router.get_backend("/not_found"), None);
    }
}
