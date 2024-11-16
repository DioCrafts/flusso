//! Ingress processor module for managing backend servers based on Ingress events.
//!
//! The `IngressProcessor` struct processes Ingress events, adding or removing
//! backend servers from the load balancer based on these events.

use std::net::SocketAddr;
use tokio::sync::mpsc;
use crate::proxy::load_balancer::LoadBalancer;
use std::sync::Arc;

/// Represents an Ingress event that either adds or removes a backend server.
#[derive(Debug)]
pub enum IngressEvent {
    Add(SocketAddr),
    Remove(SocketAddr),
}

/// Processes `IngressEvent`s to update the load balancer.
pub struct IngressProcessor {
    load_balancer: Arc<LoadBalancer>,
    event_receiver: mpsc::Receiver<IngressEvent>,
}

impl IngressProcessor {
    /// Creates a new `IngressProcessor` with a load balancer and event receiver.
    ///
    /// # Parameters
    /// - `load_balancer`: Shared `LoadBalancer` for backend management.
    /// - `event_receiver`: Receiver channel for `IngressEvent`s.
    pub fn new(load_balancer: Arc<LoadBalancer>, event_receiver: mpsc::Receiver<IngressEvent>) -> Self {
        Self {
            load_balancer,
            event_receiver,
        }
    }

    /// Processes incoming events to update the backend list in the load balancer.
    ///
    /// This function listens for `IngressEvent`s from the receiver and updates the
    /// load balancer by adding or removing backend addresses as needed.
    pub async fn process_events(&mut self) {
        while let Some(event) = self.event_receiver.recv().await {
            println!("Event received in IngressProcessor: {:?}", event);
            match event {
                IngressEvent::Add(addr) => {
                    self.load_balancer.add_backend(addr);
                    println!("Backend {} added.", addr);
                }
                IngressEvent::Remove(addr) => {
                    self.load_balancer.remove_backend(&addr);
                    println!("Backend {} removed.", addr);
                }
            }
        }
    }
}
