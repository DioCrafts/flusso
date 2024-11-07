// src/gui/gui_server.rs

use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::sync::Arc;
use crate::proxy::load_balancer::LoadBalancer;
use serde_json::json;

/// Inicia el servidor de GUI en el puerto especificado.
pub async fn start_gui_server(load_balancer: Arc<LoadBalancer>, port: u16) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(load_balancer.clone()))
            .route("/", web::get().to(index))
            .route("/backends", web::get().to(get_backends))
            .service(actix_files::Files::new("/static", "./src/gui/static").show_files_listing())
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

/// Muestra la página principal de la GUI.
async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("./static/index.html"))
}

/// Devuelve una lista de los backends actuales en formato JSON, incluyendo su estado.
async fn get_backends(data: web::Data<Arc<LoadBalancer>>) -> impl Responder {
    let backends = data.get_backends().iter().map(|backend| {
        json!({
            "address": backend.to_string(),
            "status": "active", // Puedes ajustar esto para mostrar el estado real si lo tienes.
            "connections": 0 // Esto es un placeholder; puedes agregar conexiones activas si tienes el dato.
        })
    }).collect::<Vec<_>>();
    HttpResponse::Ok().json(backends)
}

