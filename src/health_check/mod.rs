// Módulo de health checks
// src/health_check/mod.rs

pub mod backend_checker;

pub use backend_checker::check_backend_health;
