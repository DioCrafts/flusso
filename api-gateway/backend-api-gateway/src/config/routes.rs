// src/config/routes.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub id: String,
    pub path: String,
    pub target_service: String,
    pub methods: Vec<String>,
    pub auth_required: bool,
    pub rate_limit: Option<RateLimit>,
    pub timeout: Option<u64>,
    pub retry: Option<Retry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimit {
    pub requests_per_second: u32,
    pub burst_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Retry {
    pub max_attempts: u32,
    pub backoff_ms: u64,
}

#[derive(Debug)]
pub struct RouteConfig {
    routes: HashMap<String, Route>,
}

impl RouteConfig {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn add_route(&mut self, route: Route) {
        self.routes.insert(route.id.clone(), route);
    }

    pub fn get_route(&self, id: &str) -> Option<&Route> {
        self.routes.get(id)
    }

    pub fn remove_route(&mut self, id: &str) -> Option<Route> {
        self.routes.remove(id)
    }

    pub fn find_route_by_path(&self, path: &str, method: &str) -> Option<&Route> {
        self.routes.values().find(|route| {
            route.path == path && route.methods.iter().any(|m| m == method)
        })
    }

    pub fn get_all_routes(&self) -> Vec<&Route> {
        self.routes.values().collect()
    }
}