// src/ingress_controller/ingress_processor.rs

use std::net::SocketAddr;
use tokio::sync::mpsc;
use crate::proxy::load_balancer::LoadBalancer;

pub enum IngressEvent {
    Add(SocketAddr),
    Remove(SocketAddr),
}

pub struct IngressProcessor {
    load_balancer: LoadBalancer,
    event_receiver: mpsc::Receiver<IngressEvent>,
}

impl IngressProcessor {
    pub fn new(load_balancer: LoadBalancer) -> Self {
        let (_, rx) = mpsc::channel(32); // Canal de eventos compartido con el EventListener
        Self {
            load_balancer,
            event_receiver: rx,
        }
    }

    /// Procesa los eventos de Ingress para actualizar la lista de backends.
    pub async fn process_events(&mut self) {
        while let Some(event) = self.event_receiver.recv().await {
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
