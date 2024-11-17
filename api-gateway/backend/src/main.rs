//! Main entry point for the Flusso API Gateway application.
//!
//! This program initializes and starts the API Gateway server and the REST API server concurrently,
//! using asynchronous execution. Configuration settings are loaded from a configuration file.

use std::error::Error;
use flusso_api_gateway::gateway::start_gateway_api; // Gateway API logic
use flusso_api_gateway::rest::start_rest_server; // REST server logic
use futures_util::TryFutureExt;
use rustls::crypto;
use tokio::main;
use kube::Client; // Import the Kubernetes client

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Set up the default cryptographic provider at the process level.
    crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Failed to set default CryptoProvider");

    // Initialize the Kubernetes client
    let client = Client::try_default().await?;

    // Set the REST API server port
    let rest_port = 8081;

    // Start the Gateway API and REST API server concurrently.
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
        })
    )?;

    println!("API Gateway and REST server started successfully.");

    Ok(())
}
