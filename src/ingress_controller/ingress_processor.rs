// src/ingress_controller/ingress_processor.rs

use std::net::SocketAddr;
use tokio::sync::mpsc;
use crate::proxy::load_balancer::LoadBalancer;
use std::sync::Arc;

#[derive(Debug)] // Esto permite que `IngressEvent` se pueda formatear en `println!`
pub enum IngressEvent {
    Add(SocketAddr),
    Remove(SocketAddr),
}

pub struct IngressProcessor {
    load_balancer: Arc<LoadBalancer>,
    event_receiver: mpsc::Receiver<IngressEvent>,
}

impl IngressProcessor {
    // Cambiamos el constructor para recibir el receptor del canal
    pub fn new(load_balancer: Arc<LoadBalancer>, event_receiver: mpsc::Receiver<IngressEvent>) -> Self {
        Self {
            load_balancer,
            event_receiver,
        }
    }

    /// Procesa los eventos de Ingress para actualizar la lista de backends.
    pub async fn process_events(&mut self) {
        while let Some(event) = self.event_receiver.recv().await {
            println!("Evento recibido en IngressProcessor: {:?}", event);
            match event {
                IngressEvent::Add(addr) => {
                    self.load_balancer.add_backend(addr);
                    println!("Backend {} añadido.", addr);
                }
                IngressEvent::Remove(addr) => {
                    self.load_balancer.remove_backend(&addr);
                    println!("Backend {} eliminado.", addr);
                }
            }
        }
    }
}

