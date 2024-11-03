// src/ingress_controller/event_listener.rs

use tokio::sync::mpsc;
use crate::ingress_controller::ingress_processor::IngressEvent;

pub struct EventListener {
    pub event_channel: mpsc::Sender<IngressEvent>,
}

impl EventListener {
    // Cambiamos `new` para retornar también el receptor del canal
    pub fn new() -> (Self, mpsc::Receiver<IngressEvent>) {
        let (tx, rx) = mpsc::channel(32); // Canal para comunicar eventos de Ingress
        (Self { event_channel: tx }, rx)
    }

    /// Comienza a escuchar eventos de Kubernetes, enviándolos a `IngressProcessor`.
    pub async fn start_listening(&self) {
        loop {
            let event = IngressEvent::Add("127.0.0.1:8081".parse().unwrap());

            match self.event_channel.send(event).await {
                Ok(_) => println!("Evento enviado exitosamente al procesador."),
                Err(e) => eprintln!("Error al enviar evento al procesador: {:?}", e),
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        }
    }
}
