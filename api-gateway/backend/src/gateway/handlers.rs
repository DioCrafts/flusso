use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use kube::Client;

use crate::gateway::crd::gateway::{GatewaySpec, GatewayInnerSpec, GatewayManager};
use super::models::Gateway; // Modelo de Gateway compartido

/// Listar todos los Gateways disponibles en Kubernetes.
pub async fn list_gateways(client: web::Data<Client>) -> impl Responder {
    let manager = GatewayManager::new(client.get_ref().clone());

    match manager.list_gateways("default").await {
        Ok(gateways) => HttpResponse::Ok().json(gateways),
        Err(e) => {
            eprintln!("Error listing Gateways: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to list Gateways.")
        }
    }
}

/// Crear un nuevo Gateway con configuración inicial.
pub async fn add_gateway(client: web::Data<Client>, gateway: web::Json<Gateway>) -> impl Responder {
    let manager = GatewayManager::new(client.get_ref().clone());

    // Construir `GatewaySpec` basado en el modelo `Gateway`.
    let gateway_spec = GatewaySpec {
        spec: GatewayInnerSpec {
            hostname: gateway.hostname.clone(),
            tls_enabled: gateway.tls_enabled,
            certificate: gateway.certificate.clone(),
            routes: gateway.routes.iter().cloned().map(Into::into).collect(),
        },
    };
    

    match manager.create_gateway("default", &gateway_spec).await {
        Ok(_) => HttpResponse::Created().json(json!({ "message": "Gateway created successfully." })),
        Err(e) => {
            eprintln!("Error creating Gateway: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to create Gateway.")
        }
    }
}

/// Eliminar un Gateway existente por su ID.
pub async fn delete_gateway(client: web::Data<Client>, id: web::Path<String>) -> impl Responder {
    let manager = GatewayManager::new(client.get_ref().clone());

    match manager.delete_gateway("default", &id.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json(json!({ "message": "Gateway deleted successfully." })),
        Err(e) => {
            eprintln!("Error deleting Gateway: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to delete Gateway.")
        }
    }
}

/// Configurar TLS para un Gateway existente.
pub async fn configure_tls(
    client: web::Data<Client>,
    payload: web::Json<(String, String)>, // (ID del Gateway, certificado TLS)
) -> impl Responder {
    let manager = GatewayManager::new(client.get_ref().clone());
    let (id, certificate) = payload.into_inner();

    match manager.configure_tls("default", &id, &certificate).await {
        Ok(_) => HttpResponse::Ok().json(json!({ "message": "TLS configured successfully." })),
        Err(e) => {
            eprintln!("Error configuring TLS: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to configure TLS.")
        }
    }
}

/// Obtener métricas relacionadas con los Gateways.
pub async fn get_gateway_metrics(client: web::Data<Client>) -> impl Responder {
    let manager = GatewayManager::new(client.get_ref().clone());

    match manager.get_metrics().await {
        Ok(metrics) => HttpResponse::Ok().json(metrics),
        Err(e) => {
            eprintln!("Error fetching Gateway metrics: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to fetch Gateway metrics.")
        }
    }
}

/// Inicia el servicio de Gateway API.
/// Esto configura y ejecuta todos los endpoints relacionados con Gateways.
pub async fn start_gateway_api(client: Client) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Starting Gateway API...");

    // Configuración del servidor Actix-Web para manejar los endpoints.
    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(web::Data::new(client.clone()))
            .route("/gateways", web::get().to(list_gateways)) // Endpoint para listar Gateways
            .route("/gateways", web::post().to(add_gateway))  // Endpoint para crear Gateway
            .route("/gateways/{id}", web::delete().to(delete_gateway)) // Eliminar Gateway
            .route("/gateways/tls", web::post().to(configure_tls)) // Configurar TLS
            .route("/gateways/metrics", web::get().to(get_gateway_metrics)) // Obtener métricas
    })
    .bind(("0.0.0.0", 8082))? // Puerto para el Gateway API
    .run()
    .await?;

    println!("Gateway API is running on port 8082.");
    Ok(())
}
