use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::sync::Arc;
use crate::proxy::load_balancer::LoadBalancer;
use serde_json::json;

/// Endpoint para obtener la lista de Ingresses
/// En este caso, utilizamos `get_backends()` para obtener la lista de backends y los tratamos como Ingresses.
async fn get_ingresses(data: web::Data<Arc<LoadBalancer>>) -> impl Responder {
    // Usar `get_backends()` para obtener los datos
    let ingresses = data.get_ref().get_backends().iter().map(|backend| {
        json!( {
            "name": backend.to_string(),  // Convertimos el `SocketAddr` a cadena para mostrarlo
            "namespace": "default" // Asumimos un namespace por defecto
        })
    }).collect::<Vec<_>>();
    
    // Devuelve la lista de Ingresses como una respuesta JSON
    HttpResponse::Ok().json(ingresses)
}

/// Endpoint para obtener la lista de Routes
/// Usamos `get_backends()` y tratamos los backends como rutas.
async fn get_routes(data: web::Data<Arc<LoadBalancer>>) -> impl Responder {
    let routes = data.get_ref().get_backends().iter().map(|backend| {
        json!( {
            "name": backend.to_string(),  // Convertimos el `SocketAddr` a cadena
            "protocol": "HTTP",  // Asumimos el protocolo como HTTP
            "port": 80  // Usamos el puerto 80 como ejemplo
        })
    }).collect::<Vec<_>>();
    
    // Devuelve la lista de Routes como una respuesta JSON
    HttpResponse::Ok().json(routes)
}

/// Función que maneja la página principal (Dashboard)
/// Aquí deberías servir el archivo HTML de la página principal del GUI.
async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("./static/index.html"))  // Asegúrate de tener el archivo index.html en la ruta correcta.
}

/// Iniciar el servidor GUI con los endpoints adecuados
/// Aquí el servidor usa Actix Web y se configura con las rutas para Ingresses, Routes y archivos estáticos.
pub async fn start_gui_server(load_balancer: Arc<LoadBalancer>, port: u16) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(load_balancer.clone()))  // Pasa la instancia compartida de LoadBalancer
            .route("/", web::get().to(index))  // Página principal con el Dashboard
            .route("/api/ingresses", web::get().to(get_ingresses))  // Endpoint para obtener los Ingresses
            .route("/api/routes", web::get().to(get_routes))  // Endpoint para obtener los Routes
            .service(actix_files::Files::new("/static", "./static").show_files_listing())  // Archivos estáticos (CSS, JS, imágenes)
    })
    .bind(("0.0.0.0", port))?  // Asegúrate de que el puerto no esté ocupado
    .run()
    .await
}
