pub mod handlers;
pub mod routes;

pub use routes::configure_routes;

/// Inicializar el manejador de backends
pub async fn start_backends_manager() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Starting Backends Manager...");
    // Aquí iría la lógica específica de inicialización del manejador de backends
    Ok(())
}
