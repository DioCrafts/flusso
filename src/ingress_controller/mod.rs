// src/ingress_controller/mod.rs

pub mod event_listener;
pub mod ingress_processor;

use crate::proxy::HttpProxy;
use event_listener::EventListener;
use ingress_processor::IngressProcessor;

pub struct IngressController {
    event_listener: EventListener,
    ingress_processor: IngressProcessor,
    proxy: HttpProxy,
}

impl IngressController {
    pub fn new(proxy: HttpProxy) -> Self {
        let event_listener = EventListener::new();
        let ingress_processor = IngressProcessor::new();

        Self {
            event_listener,
            ingress_processor,
            proxy,
        }
    }

    /// Inicializa el controlador y comienza a escuchar los eventos de Kubernetes.
    pub async fn start(&self) {
        // Escucha y procesa eventos de Kubernetes
        self.event_listener.start_listening().await;
    }
}
