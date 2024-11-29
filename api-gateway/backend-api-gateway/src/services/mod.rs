// src/services/mod.rs
pub mod discovery;
pub mod load_balancer;

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInstance {
    pub id: String,
    pub name: String,
    pub url: String,
    pub health_check_url: String,
    pub status: ServiceStatus,
    pub metadata: ServiceMetadata,
    pub last_check: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceStatus {
    Healthy,
    Unhealthy,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMetadata {
    pub version: String,
    pub environment: String,
    pub tags: Vec<String>,
}