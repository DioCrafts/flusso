//! Main entry point for the Flusso ingress controller application.
//!
//! This program initializes and starts both the ingress controller and the GUI server
//! concurrently, using asynchronous execution. Configuration settings are loaded from
//! a configuration file, and a load balancer is set up to manage backend services.
//!
//! # Modules and Functions
//! - `Settings`: Manages application configuration settings.
//! - `LoadBalancer`: Handles backend service distribution for requests.
//! - `start_ingress_controller`: Starts the ingress controller to handle incoming requests.
//! - `start_gui_server`: Launches a GUI server for managing and monitoring backend services.

use std::sync::Arc;
use std::error::Error;
use flusso::config::settings::Settings;
use flusso::gui::start_gui_server;
use flusso::proxy::load_balancer::LoadBalancer;
use flusso::ingress_controller::start_ingress_controller;
use futures_util::TryFutureExt;
use rustls::crypto;

/// Main function of the Flusso application.
/// 
/// Initializes the cryptographic provider, loads application settings, creates a load balancer,
/// and starts both the ingress controller and the GUI server concurrently.
/// 
/// # Returns
/// - `Ok(())` if both the ingress controller and GUI server start successfully.
/// - `Err(Box<dyn Error + Send + Sync>)` if there is an error during initialization or runtime.
/// 
/// # Errors
/// - Returns an error if there are issues with setting the default cryptographic provider,
///   loading configuration settings, or running either of the main tasks (ingress controller
///   or GUI server).
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
    // The load balancer will manage request distribution across backends.
    let load_balancer = Arc::new(LoadBalancer::new(Vec::new()));
    println!("Load balancer initialized.");

    // Set the GUI server port, defaulting to 8081 if not specified in the settings.
    let gui_port = settings.gui_port.unwrap_or(8081);
    println!("The GUI server will start on port: {}", gui_port);

    // Start both the ingress controller and the GUI server concurrently.
    // Uses `tokio::try_join!` to run both tasks asynchronously and handle any errors.
    tokio::try_join!(
        // Start the ingress controller, passing in the load balancer and server address.
        start_ingress_controller(load_balancer.clone(), &settings.server_addr)
            .map_err(|e| {
                eprintln!("Error in start_ingress_controller: {:?}", e);
                Box::<dyn std::error::Error + Send + Sync>::from(e)
            }),

        // Start the GUI server, passing in the load balancer and specified port.
        start_gui_server(load_balancer.clone(), gui_port)
            .map_err(|e| {
                eprintln!("Error in start_gui_server: {:?}", e);
                Box::<dyn std::error::Error + Send + Sync>::from(e)
            })
    )?;

    println!("Both tasks completed successfully.");

    Ok(())
}
