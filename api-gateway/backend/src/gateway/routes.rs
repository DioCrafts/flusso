//! src/gateway/routes.rs
//!
//! Define RESTful routes for managing Gateways.
//!
//! These routes map to handlers that interact with Kubernetes Gateway API resources.

use actix_web::web;
use super::handlers; // Importamos los controladores desde handlers.rs

/// Configura las rutas para el módulo de Gateway.
///
/// Esta función registra todas las rutas relacionadas con Gateways en el servidor Actix-Web.
///
/// # Arguments
/// * `cfg` - Una referencia mutable a la configuración del servicio.
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/gateways") // Define el scope base para las rutas
            .route("", web::get().to(handlers::list_gateways)) // GET /api/gateways
            .route("", web::post().to(handlers::add_gateway)) // POST /api/gateways
            .route("/{id}", web::delete().to(handlers::delete_gateway)) // DELETE /api/gateways/{id}
            .route("/tls", web::post().to(handlers::configure_tls)) // POST /api/gateways/tls
            .route("/metrics", web::get().to(handlers::get_gateway_metrics)), // GET /api/gateways/metrics
    );
}
