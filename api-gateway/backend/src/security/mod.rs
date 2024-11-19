pub mod handlers;
pub mod routes;

pub use routes::configure_routes;

/// Inicializar el servicio de seguridad
pub async fn start_security_service() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Starting Security Service...");
    // Aquí iría la lógica específica de inicialización de seguridad
    Ok(())
}
