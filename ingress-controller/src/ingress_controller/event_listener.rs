//! Event listener module for watching Kubernetes Ingress resources.
//!
//! The `EventListener` struct monitors Ingress resources in a Kubernetes cluster,
//! listening for additions and removals of Ingresses, and updating the load balancer accordingly.

use kube::{api::{Api, ListParams}, Client};
use kube_runtime::watcher::{watcher, Config, Event as KubeEvent};
use tokio::sync::mpsc;
use crate::ingress_controller::ingress_processor::IngressEvent;
use crate::proxy::load_balancer::LoadBalancer;
use futures_util::{StreamExt, pin_mut};
use std::error::Error;
use std::sync::Arc;

/// Listens for Kubernetes Ingress events and sends updates to the load balancer.
#[derive(Clone, Debug)]
pub struct EventListener {
    pub event_channel: mpsc::Sender<IngressEvent>,
    pub load_balancer: Arc<LoadBalancer>,
}

impl EventListener {
    /// Creates a new `EventListener` instance and returns it along with an event receiver.
    ///
    /// # Parameters
    /// - `load_balancer`: Shared `LoadBalancer` to manage backend distribution.
    ///
    /// # Returns
    /// A tuple with `EventListener` and a receiver for `IngressEvent`s.
    pub fn new(load_balancer: Arc<LoadBalancer>) -> (Self, mpsc::Receiver<IngressEvent>) {
        let (tx, rx) = mpsc::channel(32);
        (
            Self {
                event_channel: tx,
                load_balancer,
            },
            rx,
        )
    }

    /// Starts listening for Kubernetes Ingress events, updating the load balancer.
    ///
    /// # Returns
    /// - `Ok(())` if the listener starts successfully.
    /// - `Err` if there are issues during listening or processing.
    pub async fn start_listening(&self) -> Result<(), Box<dyn Error>> {
        let client = Client::try_default().await.expect("Failed to create Kubernetes client");
        let ingresses: Api<k8s_openapi::api::networking::v1::Ingress> = Api::all(client);

        // Load existing Ingresses at startup
        if let Ok(ingress_list) = ingresses.list(&ListParams::default()).await {
            for ingress in ingress_list {
                self.process_ingress(ingress).await?;
            }
        }

        // Continuous listening for changes in Ingress
        let config = Config::default();
        let watcher_stream = watcher(ingresses, config);
        pin_mut!(watcher_stream);

        while let Some(event) = watcher_stream.next().await {
            match event {
                Ok(KubeEvent::Apply(ingress)) => self.process_ingress(ingress).await?,
                Ok(KubeEvent::Delete(ingress)) => self.remove_ingress(ingress).await?,
                _ => println!("Other event received, ignored"),
            }
        }

        Ok(())
    }

    /// Processes an Ingress event, adding the backend to the load balancer if it meets criteria.
    async fn process_ingress(&self, ingress: k8s_openapi::api::networking::v1::Ingress) -> Result<(), Box<dyn Error>> {
        if let Some(annotations) = &ingress.metadata.annotations {
            if let Some(class) = annotations.get("kubernetes.io/ingress.class") {
                if class == "flusso" {
                    if let Some(host) = ingress.spec.as_ref().and_then(|spec| {
                        spec.rules.as_ref()?.get(0)?.host.clone()
                    }) {
                        let host = host.to_string();

                        if let Some(service_name) = ingress.spec.as_ref().and_then(|spec| {
                            Some(spec.rules.as_ref()?.get(0)?
                                .http.as_ref()?.paths.get(0)?
                                .backend.service.as_ref()?.name.clone())
                        }) {
                            let service_namespace = ingress.metadata.namespace.clone().unwrap_or_default();
                            let service_key = format!("{}:{}", service_name, service_namespace);
                            println!("Detected host '{}', associated with service: {}", host, service_key);

                            // Resolve service IP and register it with the LoadBalancer
                            if let Ok(service_ip) = self.resolve_service_ip(&service_name, &service_namespace).await {
                                let backend_addr = format!("{}:80", service_ip).parse().unwrap();
                                self.load_balancer.add_backend(backend_addr);
                                println!("Backend registered for {}: {}", host, backend_addr);
                            } else {
                                eprintln!("Failed to resolve IP for service {}", service_key);
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Removes an Ingress event, deregistering the backend from the load balancer.
    async fn remove_ingress(&self, ingress: k8s_openapi::api::networking::v1::Ingress) -> Result<(), Box<dyn Error>> {
        if let Some(annotations) = &ingress.metadata.annotations {
            if let Some(class) = annotations.get("kubernetes.io/ingress.class") {
                if class == "flusso" {
                    if let Some(host) = ingress.spec.as_ref().and_then(|spec| {
                        spec.rules.as_ref()?.get(0)?.host.clone()
                    }) {
                        let host = host.to_string();

                        if let Some(service_name) = ingress.spec.as_ref().and_then(|spec| {
                            Some(spec.rules.as_ref()?.get(0)?
                                .http.as_ref()?.paths.get(0)?
                                .backend.service.as_ref()?.name.clone())
                        }) {
                            let service_namespace = ingress.metadata.namespace.clone().unwrap_or_default();

                            if let Ok(service_ip) = self.resolve_service_ip(&service_name, &service_namespace).await {
                                let backend_addr = format!("{}:80", service_ip).parse().unwrap();
                                self.load_balancer.remove_backend(&backend_addr);
                                println!("Backend removed for {}: {}", host, backend_addr);
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Resolves the IP address of a Kubernetes service by name and namespace.
    ///
    /// # Parameters
    /// - `service_name`: The name of the service.
    /// - `namespace`: The namespace in which the service is located.
    ///
    /// # Returns
    /// A `Result` with the IP address of the service or an error if it could not be resolved.
    async fn resolve_service_ip(&self, service_name: &str, namespace: &str) -> Result<String, Box<dyn Error>> {
        let client = Client::try_default().await?;
        let services: Api<k8s_openapi::api::core::v1::Service> = Api::namespaced(client, namespace);
        if let Some(service) = services.get(service_name).await.ok() {
            if let Some(cluster_ip) = service.spec.and_then(|spec| spec.cluster_ip) {
                return Ok(cluster_ip);
            }
        }
        Err(Box::from("Failed to resolve service IP"))
    }
}
