// src/models/route.rs
use serde::{Deserialize, Serialize};
use super::Metadata;
use uuid::Uuid;
use chrono::Utc;  // Añadido

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub id: String,
    pub path: String,
    pub method: HttpMethod,
    pub target_service: TargetService,
    pub auth_config: AuthConfig,
    pub proxy_config: ProxyConfig,
    pub rate_limit: Option<RateLimit>,
    pub metadata: Metadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]  // Añadido PartialEq
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetService {
    pub name: String,
    pub namespace: Option<String>,
    pub port: Option<u16>,
    pub path_rewrite: Option<String>,
    pub strip_path: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub required: bool,
    pub scopes: Vec<String>,
    pub roles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyConfig {
    pub timeout_ms: u64,
    pub retry_policy: Option<RetryPolicy>,
    pub circuit_breaker: Option<CircuitBreaker>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimit {
    pub requests_per_second: u32,
    pub burst_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    pub max_attempts: u32,
    pub backoff_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitBreaker {
    pub threshold_percentage: f32,
    pub min_requests: u32,
    pub window_size_ms: u64,
    pub recovery_time_ms: u64,
}

impl Route {
    pub fn new(
        path: String,
        method: HttpMethod,
        target_service: TargetService,
        auth_config: AuthConfig,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            path,
            method,
            target_service,
            auth_config,
            proxy_config: ProxyConfig {
                timeout_ms: 30000, // 30 segundos por defecto
                retry_policy: None,
                circuit_breaker: None,
            },
            rate_limit: None,
            metadata: Metadata {
                created_at: Utc::now(),
                updated_at: Utc::now(),
                version: 1,
            },
        }
    }

    pub fn with_rate_limit(mut self, rate_limit: RateLimit) -> Self {
        self.rate_limit = Some(rate_limit);
        self
    }

    pub fn with_retry_policy(mut self, retry_policy: RetryPolicy) -> Self {
        self.proxy_config.retry_policy = Some(retry_policy);
        self
    }

    pub fn with_circuit_breaker(mut self, circuit_breaker: CircuitBreaker) -> Self {
        self.proxy_config.circuit_breaker = Some(circuit_breaker);
        self
    }

    pub fn update_metadata(&mut self) {
        self.metadata.updated_at = Utc::now();
        self.metadata.version += 1;
    }

    pub fn matches(&self, path: &str, method: &HttpMethod) -> bool {
        self.path == path && self.method == *method
    }
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            required: false,
            scopes: vec![],
            roles: vec![],
        }
    }
}

// Implementación de pruebas
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_route() {
        let target = TargetService {
            name: "test-service".to_string(),
            namespace: None,
            port: Some(8080),
            path_rewrite: None,
            strip_path: false,
        };

        let auth = AuthConfig::default();

        let route = Route::new(
            "/api/test".to_string(),
            HttpMethod::Get,
            target,
            auth,
        );

        assert_eq!(route.path, "/api/test");
        assert!(matches!(route.method, HttpMethod::Get));
        assert_eq!(route.proxy_config.timeout_ms, 30000);
    }

    #[test]
    fn test_route_with_rate_limit() {
        let target = TargetService {
            name: "test-service".to_string(),
            namespace: None,
            port: Some(8080),
            path_rewrite: None,
            strip_path: false,
        };

        let rate_limit = RateLimit {
            requests_per_second: 100,
            burst_size: 10,
        };

        let route = Route::new(
            "/api/test".to_string(),
            HttpMethod::Get,
            target,
            AuthConfig::default(),
        ).with_rate_limit(rate_limit);

        assert!(route.rate_limit.is_some());
        assert_eq!(route.rate_limit.unwrap().requests_per_second, 100);
    }
}
