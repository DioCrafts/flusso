// Módulo de métricas
// src/metrics/mod.rs

pub mod prometheus;

use actix_web::{web, HttpResponse, Responder};
use prometheus::{Encoder, TextEncoder};
use crate::metrics::prometheus::METRICS_REGISTRY;

pub async fn metrics_handler() -> impl Responder {
    let encoder = TextEncoder::new();
    let metric_families = METRICS_REGISTRY.gather();
    let mut buffer = Vec::new();

    if let Err(e) = encoder.encode(&metric_families, &mut buffer) {
        eprintln!("Error encoding metrics: {}", e);
        return HttpResponse::InternalServerError().body("Error gathering metrics");
    }

    HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .body(buffer)
}
