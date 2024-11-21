//src/rest/routes.rs

use actix_web::web;
use crate::rest::handlers::{get_backends, add_backend, delete_backend};
use crate::gateway::handlers::{list_gateways, add_gateway, delete_gateway, configure_tls, get_gateway_metrics};
use crate::observability::handlers::{get_metrics, get_logs, export_prometheus_metrics}; // Importa handlers correctos
use crate::security::handlers::{list_policies, add_policy, delete_policy, protected_endpoint};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/metrics", web::get().to(get_metrics)) // GET /api/metrics
            .route("/logs", web::get().to(get_logs))       // GET /api/logs
            .route("/metrics/export", web::get().to(export_prometheus_metrics)) // GET /api/metrics/export

            // Backends
            .route("/backends", web::get().to(get_backends)) // GET /api/backends
            .route("/backends", web::post().to(add_backend)) // POST /api/backends
            .route("/backends/{id}", web::delete().to(delete_backend)) // DELETE /api/backends/{id}

            // Gateways
            .route("/gateways", web::get().to(list_gateways)) // GET /api/gateways
            .route("/gateways", web::post().to(add_gateway)) // POST /api/gateways
            .route("/gateways/{id}", web::delete().to(delete_gateway)) // DELETE /api/gateways/{id}
            .route("/gateways/tls", web::post().to(configure_tls)) // POST /api/gateways/tls
            .route("/gateways/metrics", web::get().to(get_gateway_metrics)) // GET /api/gateways/metrics

            // Security
            .route("/security/policies", web::get().to(list_policies)) // GET /api/security/policies
            .route("/security/policies", web::post().to(add_policy)) // POST /api/security/policies
            .route("/security/policies/{id}", web::delete().to(delete_policy)) // DELETE /api/security/policies/{id}
            .route("/security/protected", web::get().to(protected_endpoint)), // GET /api/security/protected
    );
}

