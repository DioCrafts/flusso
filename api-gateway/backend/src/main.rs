//! Main entry point for the Flusso API Gateway application.
//!
//! This program initializes and starts multiple modules of the API Gateway concurrently,
//! including Gateway API, REST API, Backends, Observability, Security, and Plugins.

use std::error::Error;
use flusso_api_gateway::gateway::start_gateway_api; // Gateway API logic
use flusso_api_gateway::rest::start_rest_server; // REST server logic
use flusso_api_gateway::backends::start_backends_manager; // Backends management
use flusso_api_gateway::observability::start_observability_service; // Observability
use flusso_api_gateway::security::start_security_service; // Security
use flusso_api_gateway::plugins::start_plugins_manager; // Plugins
use futures_util::TryFutureExt;
use rustls::crypto;
use tokio::main;
use kube::Client; // Kubernetes client for interacting with cluster resources

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Set up the default cryptographic provider at the process level.
    crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Failed to set default CryptoProvider");

    // Initialize the Kubernetes client
    let client = Client::try_default().await?;

    // Configuration for different modules
    let rest_port = 8081; // REST API port
    let observability_port = 9090; // Observability service port (Prometheus metrics, etc.)

    // Start all modules concurrently.
    tokio::try_join!(
        // Start Gateway API
        start_gateway_api(client.clone()).map_err(|e| {
            eprintln!("Error in start_gateway_api: {:?}", e);
            Box::<dyn std::error::Error + Send + Sync>::from(e)
        }),

        // Start REST API server
        start_rest_server(rest_port).map_err(|e| {
            eprintln!("Error in start_rest_server: {:?}", e);
            Box::<dyn std::error::Error + Send + Sync>::from(e)
        }),

        // Start Backends manager
        start_backends_manager(client.clone()).map_err(|e| {
            eprintln!("Error in start_backends_manager: {:?}", e);
            Box::<dyn std::error::Error + Send + Sync>::from(e)
        }),

        // Start Observability service
        start_observability_service(observability_port).map_err(|e| {
            eprintln!("Error in start_observability_service: {:?}", e);
            Box::<dyn std::error::Error + Send + Sync>::from(e)
        }),

        // Start Security service
        start_security_service(client.clone()).map_err(|e| {
            eprintln!("Error in start_security_service: {:?}", e);
            Box::<dyn std::error::Error + Send + Sync>::from(e)
        }),

        // Start Plugins manager
        start_plugins_manager(client.clone()).map_err(|e| {
            eprintln!("Error in start_plugins_manager: {:?}", e);
            Box::<dyn std::error::Error + Send + Sync>::from(e)
        })
    )?;

    println!("Flusso API Gateway and associated services started successfully.");

    Ok(())
}
