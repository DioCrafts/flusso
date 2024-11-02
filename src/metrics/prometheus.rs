// Exportador de métricas para Prometheus
// src/metrics/prometheus.rs

use prometheus::{Registry, IntCounter, IntGauge, Opts};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref METRICS_REGISTRY: Registry = Registry::new();

    // Contador de solicitudes HTTP recibidas
    pub static ref HTTP_REQUESTS_TOTAL: IntCounter = IntCounter::with_opts(
        Opts::new("http_requests_total", "Total de solicitudes HTTP recibidas")
    ).expect("Failed to create metric HTTP_REQUESTS_TOTAL");

    // Gauge para el número de conexiones activas
    pub static ref ACTIVE_CONNECTIONS: IntGauge = IntGauge::with_opts(
        Opts::new("active_connections", "Número de conexiones activas")
    ).expect("Failed to create metric ACTIVE_CONNECTIONS");

    // Contador para el total de errores
    pub static ref ERRORS_TOTAL: IntCounter = IntCounter::with_opts(
        Opts::new("errors_total", "Total de errores en el sistema")
    ).expect("Failed to create metric ERRORS_TOTAL");
}

pub fn init_metrics() {
    METRICS_REGISTRY.register(Box::new(HTTP_REQUESTS_TOTAL.clone()))
        .expect("Failed to register HTTP_REQUESTS_TOTAL");

    METRICS_REGISTRY.register(Box::new(ACTIVE_CONNECTIONS.clone()))
        .expect("Failed to register ACTIVE_CONNECTIONS");

    METRICS_REGISTRY.register(Box::new(ERRORS_TOTAL.clone()))
        .expect("Failed to register ERRORS_TOTAL");
}
