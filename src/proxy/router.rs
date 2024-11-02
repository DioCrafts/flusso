// Motor de enrutamiento y lógica de rutas
// src/proxy/router.rs

use std::collections::HashMap;
use std::net::SocketAddr;

#[derive(Clone, Debug)]
pub struct Route {
    pub path: String,
    pub backend: SocketAddr,
}

#[derive(Default)]
pub struct Router {
    routes: HashMap<String, Route>,
}

impl Router {
    /// Crea un nuevo enrutador vacío.
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    /// Agrega una nueva ruta al enrutador.
    pub fn add_route(&mut self, path: String, backend: SocketAddr) {
        let route = Route { path: path.clone(), backend };
        self.routes.insert(path, route);
    }

    /// Obtiene el backend correspondiente a una ruta.
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
