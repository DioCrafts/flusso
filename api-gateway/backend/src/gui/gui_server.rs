use actix_web::{web, HttpServer, HttpResponse, App};
use kube::{Api, Client};
use kube::api::{ListParams};
use serde_json::json;
use std::sync::Arc;
use std::error::Error;

// Asegúrate de importar el tipo Gateway desde su ubicación en tu proyecto
use crate::gateway::gateway::Gateway; // Ajusta la importación según la ubicación real de Gateway

#[derive(Debug, Clone)]
struct LoadBalancer;

impl LoadBalancer {
    // Método para obtener dinámicamente los Gateways desde el Kubernetes API
    async fn get_gateways(&self) -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
        let client = Client::try_default().await?; // Cliente de Kubernetes
        let gateways: Api<Gateway> = Api::all(client); // Usamos el tipo Gateway

        // Listar recursos de tipo Gateway
        let params = ListParams::default();
        let gateway_list = gateways.list(&params).await?;

        // Mapea los recursos de tipo Gateway a una lista simple con sus nombres
        let gateways = gateway_list.items.iter()
            .filter_map(|g| g.metadata.name.clone()) // Usamos `filter_map` para eliminar `None`
            .collect::<Vec<String>>();

        Ok(gateways)
    }
}

// Función para obtener la lista de Gateways dinámicamente
async fn get_gateways_endpoint(data: web::Data<Arc<LoadBalancer>>) -> HttpResponse {
    match data.get_gateways().await {
        Ok(gateways) => HttpResponse::Ok().json(gateways),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching Gateways"),
    }
}

// Función principal para iniciar el servidor y manejar los endpoints
pub async fn start_gui_server(port: u16) -> Result<(), Box<dyn Error + Send + Sync>> {
    let addr = format!("0.0.0.0:{}", port);
    println!("Starting GUI server at http://{}", addr);

    let load_balancer = Arc::new(LoadBalancer); // Instancia del LoadBalancer

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(load_balancer.clone()))  // Pasa la instancia compartida de LoadBalancer
            .route("/", web::get().to(gui_home))  // Página principal con el Dashboard
            .route("/api/gateways", web::get().to(get_gateways_endpoint))  // Endpoint para obtener los Gateways dinámicamente
    })
    .bind(addr)?
    .run()
    .await?;

    Ok(())
}

async fn gui_home() -> HttpResponse {
    HttpResponse::Ok().body("Welcome to the Flusso GUI")
}
