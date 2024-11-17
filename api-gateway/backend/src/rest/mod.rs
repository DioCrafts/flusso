//! REST API Module for Flusso
//!
//! This module handles all REST endpoints for the backend, exposing data
//! for the frontend and other clients.

use actix_web::{App, HttpServer};
use crate::rest::routes::configure_routes;

/// Starts the REST API server.
///
/// # Arguments
/// - `port`: Port number for the REST server.
///
/// # Returns
/// A `Result` indicating whether the server started successfully.
pub async fn start_rest_server(port: u16) -> std::io::Result<()> {
    println!("Starting REST server on port {}", port);

    HttpServer::new(|| {
        App::new()
            .configure(configure_routes) // Register routes
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

pub mod handlers;
pub mod routes;
