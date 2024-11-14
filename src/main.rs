//! Main entry point for the Flusso ingress controller application.
//!
//! This program initializes and starts the ingress controller, GUI server, and Gateway API
//! concurrently, using asynchronous execution. Configuration settings are loaded from a configuration file,
//! and a load balancer is set up to manage backend services.

use std::sync::Arc;
use std::error::Error;
use flusso::config::settings::Settings;
use flusso::gui::start_gui_server;
use flusso::proxy::load_balancer::LoadBalancer;
use flusso::ingress_controller::start_ingress_controller;
use flusso::gateway::start_gateway_api; // New import for Gateway API
use kube::Client;
use futures_util::TryFutureExt;
use rustls::crypto;

/// Main function of the Flusso application.
///
/// Initializes the cryptographic provider, loads application settings, creates a load balancer,
/// and starts the ingress controller, GUI server, and Gateway API concurrently.
///
/// # Returns
/// - `Ok(())` if all components start successfully.
/// - `Err(Box<dyn Error + Send + Sync>)` if there is an error during initialization or runtime.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Set up the default cryptographic provider at the process level.
    crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Failed to set default CryptoProvider");

    // Load the application settings from the configuration file.
    let settings = Settings::new().expect("Failed to load configuration");
    println!("Configuration loaded: {:?}", settings);

    // Initialize the load balancer with an empty list of backends.
    let load_balancer = Arc::new(LoadBalancer::new(Vec::new()));
    println!("Load balancer initialized.");

    // Set the GUI server port, defaulting to 8081 if not specified in the settings.
    let gui_port = settings.gui_port.unwrap_or(8081);
    println!("The GUI server will start on port: {}", gui_port);

    // Initialize the Kubernetes client
    let client = Client::try_default().await?;

    // Start the Gateway API, ingress controller, and GUI server concurrently.
    tokio::try_join!(
        // Start the ingress controller.
        start_ingress_controller(load_balancer.clone(), &settings.server_addr)
            .map_err(|e| {
                eprintln!("Error in start_ingress_controller: {:?}", e);
                Box::<dyn std::error::Error + Send + Sync>::from(e)
            }),

        // Start the GUI server.
        start_gui_server(load_balancer.clone(), gui_port)
            .map_err(|e| {
                eprintln!("Error in start_gui_server: {:?}", e);
                Box::<dyn std::error::Error + Send + Sync>::from(e)
            }),

        // Start the Gateway API manager.
        start_gateway_api(client.clone()).map_err(|e| {
            eprintln!("Error in start_gateway_api: {:?}", e);
            Box::<dyn std::error::Error + Send + Sync>::from(e)
        })            
    )?;

    println!("All components started successfully.");

    Ok(())
}
