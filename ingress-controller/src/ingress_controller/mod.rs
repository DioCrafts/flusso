//! Ingress Controller Module for Flusso
//!
//! This module manages the ingress controller for the Flusso application,
//! orchestrating events related to Kubernetes ingress resources and directing traffic to backends.
//! The module includes functionalities for setting up an HTTP proxy, processing ingress events,
//! and managing backend load balancing.

pub mod event_listener;
pub mod ingress_processor;

use crate::proxy::{HttpProxy, load_balancer::LoadBalancer};
use event_listener::EventListener;
use ingress_processor::IngressProcessor;
use std::sync::Arc;
use tokio::task::LocalSet;
use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse};
use bytes::Bytes;  // For handling request bodies as bytes in forward_request
use reqwest::Method;  // Import HTTP methods from reqwest
use reqwest::header::{HeaderMap as ReqwestHeaderMap, HeaderName};
use futures_util::TryFutureExt;

/// The main struct for the Ingress Controller, which manages events from Kubernetes
/// and forwards incoming HTTP requests to the appropriate backend.
pub struct IngressController {
    event_listener: EventListener,
    ingress_processor: IngressProcessor,
    proxy: HttpProxy,
}

impl IngressController {
    /// Creates a new instance of `IngressController`.
    ///
    /// # Parameters
    /// - `load_balancer`: An `Arc` reference to a shared `LoadBalancer` instance.
    ///
    /// # Returns
    /// An instance of `IngressController` initialized with an event listener, an ingress processor,
    /// and an HTTP proxy.
    pub fn new(load_balancer: Arc<LoadBalancer>) -> Self {
        println!("Initializing IngressController...");

        // Initialize EventListener with the load balancer and obtain the sender/receiver channel.
        let (event_listener, rx) = EventListener::new(load_balancer.clone());
        println!("EventListener initialized.");

        // Create IngressProcessor and pass the receiver channel for event processing.
        let ingress_processor = IngressProcessor::new(load_balancer.clone(), rx);
        println!("IngressProcessor initialized.");

        // Initialize HTTP Proxy with the load balancer.
        let proxy = HttpProxy::new(load_balancer);
        println!("HttpProxy initialized.");

        println!("IngressController fully initialized.");

        Self {
            event_listener,
            ingress_processor,
            proxy,
        }
    }

    /// Starts the EventListener to listen to Kubernetes events.
    ///
    /// Spawns a background task to continuously listen for ingress-related events and updates.
    pub async fn start(&self) {
        println!("Starting to listen to events with EventListener...");
        let listener = self.event_listener.clone();
    
        let _ = tokio::spawn(async move {
            let result = listener.start_listening().await;
            if let Err(e) = result {
                eprintln!("Error in EventListener: {:?}", e);
            }
        });
    
        println!("EventListener is actively listening to events.");
    }
    
    
    /// Processes ingress events with the IngressProcessor.
    ///
    /// Continuously listens for events and updates the load balancer's backend list accordingly.
    pub async fn process_events(&mut self) {
        self.ingress_processor.process_events().await;
    }
}

/// Starts the IngressController and manages all related tasks.
///
/// This function initiates the `IngressController`, the HTTP server for routing traffic, and the
/// background tasks for event listening and processing. It handles any initialization or runtime errors
/// by printing detailed messages to the console.
///
/// # Parameters
/// - `load_balancer`: Shared `LoadBalancer` instance for managing backend traffic.
/// - `server_addr`: The address on which the HTTP server will listen.
///
/// # Returns
/// A `Result<(), Box<dyn std::error::Error + Send + Sync>>` indicating success or error.
pub async fn start_ingress_controller(
    load_balancer: Arc<LoadBalancer>,
    server_addr: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Starting ingress controller on {}", server_addr);

    let server_addr = server_addr.to_string();
    let load_balancer_clone = load_balancer.clone();
    let mut controller = IngressController::new(load_balancer);

    // Start listening for events in a background task
    let start_task = tokio::spawn({
        let listener = controller.event_listener.clone();
        async move {
            if let Err(e) = listener.start_listening().await {
                eprintln!("Error in EventListener: {:?}", e);
            }
        }
    })
    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>);

    // Start processing events in another background task
    let process_task = tokio::spawn(async move {
        controller.process_events().await;
    })
    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>);

    // Creates a persistent LocalSet instance for the HTTP server
    let local_set = LocalSet::new();
    let server_addr_clone = server_addr.clone();

    let http_server_task = local_set
        .run_until(async move {
            HttpServer::new(move || {
                let http_proxy = HttpProxy::new(load_balancer_clone.clone());
                App::new()
                    .app_data(web::Data::new(http_proxy))
                    .default_service(web::route().to(forward_request))
            })
            .bind(server_addr_clone)?
            .run()
            .await
        })
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>);

    // Run all tasks concurrently and print completion message on success
    tokio::try_join!(start_task, process_task, http_server_task)?;

    println!("Ingress controller started successfully on {}", server_addr);
    Ok(())
}

/// Forwards incoming HTTP requests to the backend using `HttpProxy`.
///
/// # Parameters
/// - `req`: The incoming HTTP request.
/// - `body`: The request body as bytes.
/// - `proxy`: A data reference to the `HttpProxy` instance.
///
/// # Returns
/// A `HttpResponse` containing the response from the backend server, or an internal error if the forwarding fails.
async fn forward_request(
    req: HttpRequest,
    body: Bytes,
    proxy: web::Data<HttpProxy>,
) -> HttpResponse {
    let path = req.uri().path().to_string();
    let method = match req.method() {
        &actix_web::http::Method::GET => Method::GET,
        &actix_web::http::Method::POST => Method::POST,
        &actix_web::http::Method::PUT => Method::PUT,
        &actix_web::http::Method::DELETE => Method::DELETE,
        _ => Method::GET, // Default method
    };

    // Convert Actix-web headers to Reqwest headers
    let mut headers = ReqwestHeaderMap::new();
    for (key, value) in req.headers().iter() {
        if let Ok(header_name) = HeaderName::from_bytes(key.as_str().as_bytes()) {
            if let Ok(header_value_str) = value.to_str() {
                if let Ok(header_value) = reqwest::header::HeaderValue::from_str(header_value_str) {
                    headers.insert(header_name, header_value);
                }
            }
        }
    }

    println!("Forwarding request to path: {}", path);

    // Forward the request to the backend through HttpProxy
    match proxy.forward_request(&path, method, headers, Some(body)).await {
        Ok(response) => {
            // Convert `reqwest` status code to `actix_web` status code
            let status = actix_web::http::StatusCode::from_u16(response.status().as_u16())
                .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
            let body = response.text().await.unwrap_or_default();
            HttpResponse::build(status).body(body)
        }
        Err(_) => HttpResponse::InternalServerError().body("Error forwarding request"),
    }
}
