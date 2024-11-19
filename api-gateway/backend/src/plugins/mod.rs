pub mod routes;

pub use routes::configure_routes;

/// Inicializar el manejador de plugins
pub async fn start_plugins_manager() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Starting Plugins Manager...");
    // Aquí iría la lógica específica de inicialización de plugins
    Ok(())
}
