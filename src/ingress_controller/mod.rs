pub mod event_listener;
pub mod ingress_processor;

use crate::proxy::HttpProxy;
use crate::proxy::load_balancer::LoadBalancer;
use event_listener::EventListener;
use ingress_processor::{IngressEvent, IngressProcessor};
use std::sync::Arc;
use tokio::sync::Mutex;
use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse};
use bytes::Bytes;  // Importa el tipo Bytes que se usa en forward_request
use reqwest::Method;  // Importa el tipo Method desde reqwest
use reqwest::header::{HeaderMap as ReqwestHeaderMap, HeaderName};

pub struct IngressController {
    event_listener: EventListener,
    ingress_processor: IngressProcessor,
    proxy: HttpProxy,
}

impl IngressController {
    pub fn new(load_balancer: Arc<LoadBalancer>) -> Self {
        println!("Inicializando IngressController...");

        // Inicializamos EventListener con el canal y obtenemos tanto el transmisor como el receptor
        let (event_listener, rx) = EventListener::new();
        println!("EventListener inicializado.");

        // Creamos IngressProcessor y le pasamos el receptor para procesar eventos
        let ingress_processor = IngressProcessor::new(load_balancer.clone(), rx);
        println!("IngressProcessor inicializado.");

        let proxy = HttpProxy::new(load_balancer);
        println!("HttpProxy inicializado.");

        println!("IngressController inicializado completamente.");

        Self {
            event_listener,
            ingress_processor,
            proxy,
        }
    }

    /// Inicializa el controlador y comienza a escuchar los eventos de Kubernetes.
// src/ingress_controller/mod.rs
    pub async fn start(&self) {
        println!("Comenzando a escuchar eventos con el EventListener...");
        let listener = self.event_listener.clone(); // Clonamos el EventListener para evitar problemas de propiedad
        tokio::spawn(async move {
            if let Err(e) = listener.start_listening().await {
                eprintln!("Error en EventListener: {:?}", e);
            }
        });
        println!("EventListener escuchando eventos.");
    }

    /// Procesa eventos de Ingress con el IngressProcessor
    pub async fn process_events(&mut self) {
        self.ingress_processor.process_events().await;
    }
}

// Nueva función para iniciar el IngressController con manejo de errores y registros detallados
pub async fn start_ingress_controller(load_balancer: Arc<LoadBalancer>, server_addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Iniciando el controlador de ingreso en {}", server_addr);
    
    // Convertimos server_addr en String para usar en el tokio::spawn
    let server_addr = server_addr.to_string();
    let server_addr_for_task = server_addr.clone(); // Clonar server_addr para el uso en el bloque async
    
    // Clonamos `load_balancer` para pasarlo al HttpProxy
    let load_balancer_clone = load_balancer.clone();

    // Creamos el controlador
    let mut controller = IngressController::new(load_balancer);

    let start_task = tokio::spawn({
        let listener = controller.event_listener.clone();
        async move {
            listener.start_listening().await;
        }
    });

    let process_task = tokio::spawn(async move {
        controller.process_events().await;
    });

    // Configuramos el servidor Actix usando actix_web::rt::spawn para asegurar que sea `Send`
    let http_server_task = actix_web::rt::spawn(async move {
        HttpServer::new(move || {
            let http_proxy = HttpProxy::new(load_balancer_clone.clone());

            App::new()
                .app_data(web::Data::new(http_proxy))
                .default_service(
                    web::route().to(forward_request)
                )
        })
        .bind(server_addr_for_task)? // Usamos la versión clonada
        .run()
        .await
    });

    // Esperamos a que todas las tareas finalicen
    let _ = tokio::try_join!(start_task, process_task, http_server_task)?;

    println!("Controlador de ingreso iniciado exitosamente en {}", server_addr);
    Ok(())
}

// Define la función `forward_request` que utiliza HttpProxy para reenviar la solicitud
async fn forward_request(
    req: HttpRequest,
    body: Bytes,
    proxy: web::Data<HttpProxy>
) -> HttpResponse {
    let path = req.uri().path().to_string();
    let method = match req.method() {
        &actix_web::http::Method::GET => Method::GET,
        &actix_web::http::Method::POST => Method::POST,
        &actix_web::http::Method::PUT => Method::PUT,
        &actix_web::http::Method::DELETE => Method::DELETE,
        _ => Method::GET, // Método predeterminado
    };

    // Convierte los encabezados de Actix a los de Reqwest
    let mut headers = ReqwestHeaderMap::new();
    for (key, value) in req.headers().iter() {
        if let Ok(header_name) = HeaderName::from_bytes(key.as_str().as_bytes()) {
            if let Ok(header_value) = value.clone().try_into() {
                headers.insert(header_name, header_value);
            }
        }
    }

    println!("Reenviando solicitud a la ruta: {}", path);

    // Reenvía la solicitud con todos los detalles a HttpProxy
    match proxy.forward_request(&path, method, headers, Some(body)).await {
        Ok(response) => {
            // Convertir la respuesta del backend en una respuesta de Actix
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            HttpResponse::build(status).body(body)
        },
        Err(_) => HttpResponse::InternalServerError().body("Error al reenviar solicitud"),
    }
}


