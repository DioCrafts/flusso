// src/ingress_controller/mod.rs

pub mod event_listener;
pub mod ingress_processor;

use crate::proxy::HttpProxy;
use crate::proxy::load_balancer::LoadBalancer;
use event_listener::EventListener;
use ingress_processor::IngressProcessor;
use std::sync::Arc;

pub struct IngressController {
    event_listener: EventListener,
    ingress_processor: IngressProcessor,
    proxy: HttpProxy,
}

impl IngressController {
    pub fn new(load_balancer: Arc<LoadBalancer>) -> Self {
        let event_listener = EventListener::new();
        let ingress_processor = IngressProcessor::new(load_balancer.clone());
        let proxy = HttpProxy::new(load_balancer);  // Usa `Arc<LoadBalancer>` directamente

        Self {
            event_listener,
            ingress_processor,
            proxy,
        }
    }

    /// Inicializa el controlador y comienza a escuchar los eventos de Kubernetes.
    pub async fn start(&self) {
        self.event_listener.start_listening().await;
    }
}

// Nueva función para iniciar el IngressController
pub async fn start_ingress_controller(load_balancer: Arc<LoadBalancer>) -> Result<(), Box<dyn std::error::Error>> {
    let controller = IngressController::new(load_balancer);  // Pasamos `Arc<LoadBalancer>` directamente
    controller.start().await;
    Ok(())
}

