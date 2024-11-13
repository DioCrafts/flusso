//! GUI server module for displaying backend status and connection metrics.
//!
//! The `start_gui_server` function initializes an Actix web server to serve the main dashboard page
//! and backend status data in JSON format.

use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::sync::Arc;
use crate::proxy::load_balancer::LoadBalancer;
use serde_json::json;

/// Starts the GUI server on the specified port.
///
/// This server provides endpoints to:
/// - Serve the main HTML dashboard at `/`.
/// - Provide backend status data in JSON format at `/backends`.
/// - Serve static files (JavaScript, CSS) from the `/static` directory.
///
/// # Parameters
/// - `load_balancer`: Shared `LoadBalancer` instance for managing backend distribution.
/// - `port`: Port number for the GUI server.
///
/// # Returns
/// A `std::io::Result<()>` indicating success or failure.
pub async fn start_gui_server(load_balancer: Arc<LoadBalancer>, port: u16) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(load_balancer.clone()))
            .route("/", web::get().to(index))
            .route("/backends", web::get().to(get_backends))
            .service(actix_files::Files::new("/static", "/usr/local/bin/static").show_files_listing())
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

/// Serves the main HTML page for the GUI dashboard.
async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("./static/index.html"))
}

/// Returns a JSON list of current backends, including status and connections.
///
/// This endpoint provides real-time backend data, which the frontend can use to
/// display backend status and active connections.
///
/// # Returns
/// A JSON response with backend details.
async fn get_backends(data: web::Data<Arc<LoadBalancer>>) -> impl Responder {
    let backends = data.get_backends().iter().map(|backend| {
        json!({
            "address": backend.to_string(),
            "status": "active", // Placeholder for actual backend status.
            "connections": 0    // Placeholder for active connections.
        })
    }).collect::<Vec<_>>();
    HttpResponse::Ok().json(backends)
}
