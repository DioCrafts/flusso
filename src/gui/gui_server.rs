use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::sync::Arc;
use crate::proxy::load_balancer::LoadBalancer;
use serde_json::json;

pub async fn start_gui_server(load_balancer: Arc<LoadBalancer>, port: u16) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(load_balancer.clone()))
            .route("/", web::get().to(index))
            .route("/backends", web::get().to(get_backends))
            .route("/gateways", web::get().to(get_gateways)) // New route for Gateway data
            .service(actix_files::Files::new("/static", "/usr/local/bin/static").show_files_listing())
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("./static/index.html"))
}

async fn get_backends(data: web::Data<Arc<LoadBalancer>>) -> impl Responder {
    let backends = data.get_backends().iter().map(|backend| {
        json!({ "address": backend.to_string(), "status": "active", "connections": 0 })
    }).collect::<Vec<_>>();
    HttpResponse::Ok().json(backends)
}

// New function for serving Gateway data
async fn get_gateways() -> impl Responder {
    // Mocked data. Replace this with actual Gateway API integration.
    let gateways = vec![
        json!({ "name": "gateway-1", "status": "active", "routes": 5 }),
        json!({ "name": "gateway-2", "status": "inactive", "routes": 2 }),
    ];
    HttpResponse::Ok().json(gateways)
}
