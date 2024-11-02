// src/ingress_controller/event_listener.rs

use tokio::sync::mpsc;
use crate::ingress_controller::ingress_processor::IngressEvent;

pub struct EventListener {
    event_channel: mpsc::Sender<IngressEvent>,
}

impl EventListener {
    pub fn new() -> Self {
        let (tx, _rx) = mpsc::channel(32); // Canal para comunicar eventos de Ingress
        Self { event_channel: tx }
    }

    /// Comienza a escuchar eventos de Kubernetes, enviándolos a `IngressProcessor`.
    pub async fn start_listening(&self) {
        // Simulación de recepción de eventos.
        loop {
            let event = IngressEvent::Add("127.0.0.1:8081".parse().unwrap());
            if let Err(_) = self.event_channel.send(event).await {
                println!("Failed to send event to processor.");
            }

            // Simular espera entre eventos
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        }
    }
}

