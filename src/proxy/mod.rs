// Módulo del proxy principal
// src/proxy/mod.rs

pub mod cache;
pub mod router;
pub mod http;
pub mod load_balancer;

pub use cache::Cache;
pub use router::Router;
pub use http::HttpProxy;
pub use load_balancer::LoadBalancer;
