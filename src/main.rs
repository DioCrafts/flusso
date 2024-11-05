use std::sync::Arc;
use std::error::Error;
use flusso::config::settings::Settings;
use flusso::gui::start_gui_server;
use flusso::proxy::load_balancer::LoadBalancer;
use flusso::ingress_controller::start_ingress_controller;
use futures_util::TryFutureExt;
use rustls::crypto; // Importa el módulo de cripto

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Configura el proveedor criptográfico predeterminado a nivel de proceso
    crypto::aws_lc_rs::default_provider().install_default()
        .expect("Failed to set default CryptoProvider");

    let settings = Settings::new().expect("Failed to load configuration");
    println!("Configuración cargada: {:?}", settings);

    let load_balancer = Arc::new(LoadBalancer::new(Vec::new()));
    println!("Balanceador de carga inicializado.");

    let gui_port = settings.gui_port.unwrap_or(8081);
    println!("El servidor GUI se iniciará en el puerto: {}", gui_port);

    // Inicia el controlador de ingreso y el servidor GUI concurrentemente
    tokio::try_join!(
        start_ingress_controller(load_balancer.clone()).map_err(|e| {
            eprintln!("Error en start_ingress_controller: {:?}", e);
            Box::<dyn Error>::from(e)
        }),
        start_gui_server(load_balancer.clone(), gui_port).map_err(|e| {
            eprintln!("Error en start_gui_server: {:?}", e);
            Box::<dyn Error>::from(e)
        })
    )?;

    println!("Tareas ejecutadas exitosamente.");

    Ok(())
}

