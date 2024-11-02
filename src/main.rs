// src/main.rs

use actix_web::rt::System;
use std::sync::Arc;

use bilancia::config::settings::Settings;
use bilancia::ingress_controller::start_ingress_controller;
use bilancia::gui::start_gui_server;
use bilancia::proxy::load_balancer::LoadBalancer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Carga la configuración de la aplicación
    let settings = Settings::new().expect("Failed to load configuration");

    // Inicializa el balanceador de carga
    let load_balancer = Arc::new(LoadBalancer::new());

    // Inicia el controlador de Ingress
    let ingress_controller = start_ingress_controller(load_balancer.clone());
    
    // Inicia el servidor de la GUI en el puerto especificado
    let gui_port = settings.gui_port.unwrap_or(8081);
    let gui_server = start_gui_server(load_balancer.clone(), gui_port);

    // Ejecuta ambas tareas concurrentemente
    System::new().block_on(async {
        tokio::try_join!(ingress_controller, gui_server)?;
        Ok(())
    })
}
