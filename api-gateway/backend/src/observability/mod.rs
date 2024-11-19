pub mod handlers;
pub mod routes;

pub use routes::configure_routes;

/// Inicializar el servicio de observabilidad
pub async fn start_observability_service(port: u16) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Starting Observability Service on port {}...", port);
    // Aquí iría la lógica específica de inicialización de observabilidad
    Ok(())
}
