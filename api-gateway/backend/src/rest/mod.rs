use actix_web::{App, HttpServer};
use actix_web::web::{Data, ServiceConfig};
use std::sync::Mutex;
use kube::Client; // Importa el cliente de Kubernetes

pub mod routes; // Declarar submódulo de rutas
pub mod handlers; // Declarar submódulo de handlers

use crate::rest::routes::configure_routes;
use crate::backends::handlers::AppState as BackendsState;
use crate::security::handlers::AppState as SecurityState;
use crate::observability::handlers::ObservabilityState;

pub async fn start_rest_server(port: u16, client: Client) -> std::io::Result<()> {
    println!("Starting REST server on port {}", port);

    let backends_state = Data::new(BackendsState {
        backends: Mutex::new(vec![]),
    });

    let security_state = Data::new(SecurityState {
        policies: Mutex::new(vec![]),
    });

    let observability_state = Data::new(ObservabilityState::new()); // Usa el método new()

    // Si necesitas usar el cliente `client` en el futuro, puedes incluirlo aquí.

    HttpServer::new(move || {
        App::new()
            .app_data(backends_state.clone())
            .app_data(security_state.clone())
            .app_data(observability_state.clone())
            // Puedes registrar el cliente como parte del estado compartido si es necesario.
            .app_data(Data::new(client.clone()))
            .configure(|cfg: &mut ServiceConfig| configure_routes(cfg))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
