//! Route configuration for the REST API

use actix_web::web;
use crate::rest::handlers;

/// Configures the REST API routes.
///
/// This function is used to register all the available endpoints for the REST API.
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/metrics", web::get().to(handlers::get_metrics)) // GET /api/metrics
            .route("/alerts", web::get().to(handlers::get_alerts)) // GET /api/alerts
            .route("/backends", web::get().to(handlers::get_backends)) // GET /api/backends
            .route("/backends", web::post().to(handlers::add_backend)) // POST /api/backends
            .route("/backends/{id}", web::delete().to(handlers::delete_backend)), // DELETE /api/backends/{id}
    );
}

