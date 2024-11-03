use std::sync::Arc;
use std::error::Error;
use flusso::config::settings::Settings;
use flusso::gui::start_gui_server;
use flusso::proxy::load_balancer::LoadBalancer;
use flusso::ingress_controller::start_ingress_controller;
use futures_util::TryFutureExt; // Importamos TryFutureExt para usar map_err

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Carga la configuración de la aplicación
    let settings = Settings::new().expect("Failed to load configuration");

    // Inicializa el balanceador de carga con un vector vacío de backends
    let load_balancer = Arc::new(LoadBalancer::new(Vec::new()));

    // Definimos el puerto para el servidor GUI desde la configuración
    let gui_port = settings.gui_port.unwrap_or(8081);

    // Ejecuta ambas tareas concurrentemente usando tokio::try_join!
    tokio::try_join!(
        start_ingress_controller(load_balancer.clone()).map_err(|e| Box::<dyn Error>::from(e)),
        start_gui_server(load_balancer.clone(), gui_port).map_err(|e| Box::<dyn Error>::from(e))
    )?;

    Ok(())
}

