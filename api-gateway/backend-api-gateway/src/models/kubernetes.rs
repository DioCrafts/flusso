// src/models/kubernetes.rs
use k8s_openapi::api::core::v1::Service;
use kube::CustomResource;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    group = "gateway.flusso.io",
    version = "v1alpha1",
    kind = "GatewayRoute",
    namespaced
)]
pub struct GatewayRouteSpec {
    pub path: String,
    pub service: ServiceBackend,
    pub methods: Vec<String>,
    pub rate_limit: Option<RateLimit>,
    pub cors: Option<CorsConfig>,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct ServiceBackend {
    pub name: String,
    pub port: i32,
    pub namespace: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct RateLimit {
    pub requests_per_second: i32,
    pub burst_size: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
    pub allowed_methods: Vec<String>,
    pub allowed_headers: Vec<String>,
    pub max_age: Option<i32>,
}

// El tipo GatewayRoute se genera automáticamente por el macro CustomResource