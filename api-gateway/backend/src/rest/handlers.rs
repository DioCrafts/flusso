//! REST API Handlers
//!
//! These functions handle incoming HTTP requests and return appropriate responses.

use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

/// Handler for `GET /api/metrics`
///
/// Returns a list of metrics (mocked example data for now).
pub async fn get_metrics() -> impl Responder {
    let metrics = json!([
        { "timestamp": "2024-11-17T12:00:00Z", "rps": 150, "latency": 200, "errors": 5 },
        { "timestamp": "2024-11-17T12:01:00Z", "rps": 160, "latency": 180, "errors": 3 }
    ]);
    HttpResponse::Ok().json(metrics)
}

/// Handler for `GET /api/backends`
///
/// Returns a list of backend services.
pub async fn get_backends() -> impl Responder {
    let backends = json!([
        { "id": 1, "name": "Backend 1", "address": "192.168.1.10", "port": 80, "status": "Healthy" },
        { "id": 2, "name": "Backend 2", "address": "192.168.1.11", "port": 8080, "status": "Unhealthy" }
    ]);
    HttpResponse::Ok().json(backends)
}

/// Handler for `POST /api/backends`
///
/// Adds a new backend service.
pub async fn add_backend(backend: web::Json<serde_json::Value>) -> impl Responder {
    println!("Adding new backend: {:?}", backend);
    HttpResponse::Created().json(json!({ "message": "Backend added", "backend": backend }))
}

/// Handler for `DELETE /api/backends/{id}`
///
/// Deletes a backend service by ID.
pub async fn delete_backend(path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner(); // Extraer el valor de Path
    println!("Deleting backend with ID: {}", id);
    HttpResponse::Ok().json(json!({ "message": "Backend deleted", "id": id }))
}

/// Handler for `GET /api/alerts`
///
/// Returns a list of alerts (mocked example data for now).
pub async fn get_alerts() -> impl Responder {
    let alerts = json!([
        { "id": 1, "message": "High latency detected" },
        { "id": 2, "message": "Backend unreachable" }
    ]);
    HttpResponse::Ok().json(alerts)
}
