use kube::{api::{Api, ListParams}, Client};
use kube_runtime::watcher::{watcher, Config, Event as KubeEvent};
use tokio::sync::mpsc;
use crate::ingress_controller::ingress_processor::IngressEvent;
use crate::proxy::load_balancer::LoadBalancer;
use futures_util::{StreamExt, pin_mut};
use std::error::Error;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct EventListener {
    pub event_channel: mpsc::Sender<IngressEvent>,
    pub load_balancer: Arc<LoadBalancer>,
}

impl EventListener {
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

    pub async fn start_listening(&self) -> Result<(), Box<dyn Error>> {
        let client = Client::try_default().await.expect("Fallo al crear cliente de Kubernetes");
        let ingresses: Api<k8s_openapi::api::networking::v1::Ingress> = Api::all(client);

        // Cargar Ingresses existentes
        if let Ok(ingress_list) = ingresses.list(&ListParams::default()).await {
            for ingress in ingress_list {
                self.process_ingress(ingress).await?;
            }
        }

        // Escucha continua para cambios en Ingress
        let config = Config::default();
        let watcher_stream = watcher(ingresses, config);
        pin_mut!(watcher_stream);

        while let Some(event) = watcher_stream.next().await {
            match event {
                Ok(KubeEvent::Apply(ingress)) => self.process_ingress(ingress).await?,
                Ok(KubeEvent::Delete(ingress)) => self.remove_ingress(ingress).await?,
                _ => println!("Otro tipo de evento recibido, ignorado"),
            }
        }

        Ok(())
    }

    async fn process_ingress(&self, ingress: k8s_openapi::api::networking::v1::Ingress) -> Result<(), Box<dyn Error>> {
        if let Some(annotations) = &ingress.metadata.annotations {
            if let Some(class) = annotations.get("kubernetes.io/ingress.class") {
                if class == "flusso" {
                    if let Some(host) = ingress.spec.as_ref().and_then(|spec| {
                        spec.rules.as_ref()?.get(0)?.host.clone()
                    }) {
                        let host = host.to_string(); // Convertir a String

                        if let Some(service_name) = ingress.spec.as_ref().and_then(|spec| {
                            Some(spec.rules.as_ref()?.get(0)?
                                .http.as_ref()?.paths.get(0)?
                                .backend.service.as_ref()?.name.clone())
                        }) {
                            let service_namespace = ingress.metadata.namespace.clone().unwrap_or_default();
                            let service_key = format!("{}:{}", service_name, service_namespace);
                            println!("Detectado host '{}', asociado al servicio: {}", host, service_key);

                            // Resolución del servicio y registro en el LoadBalancer
                            if let Ok(service_ip) = self.resolve_service_ip(&service_name, &service_namespace).await {
                                let backend_addr = format!("{}:80", service_ip).parse().unwrap();
                                self.load_balancer.add_backend(backend_addr);
                                println!("Backend registrado para {}: {}", host, backend_addr);
                            } else {
                                eprintln!("No se pudo resolver el IP para el servicio {}", service_key);
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    async fn remove_ingress(&self, ingress: k8s_openapi::api::networking::v1::Ingress) -> Result<(), Box<dyn Error>> {
        if let Some(annotations) = &ingress.metadata.annotations {
            if let Some(class) = annotations.get("kubernetes.io/ingress.class") {
                if class == "flusso" {
                    if let Some(host) = ingress.spec.as_ref().and_then(|spec| {
                        spec.rules.as_ref()?.get(0)?.host.clone()
                    }) {
                        let host = host.to_string(); // Convertir a String

                        if let Some(service_name) = ingress.spec.as_ref().and_then(|spec| {
                            Some(spec.rules.as_ref()?.get(0)?
                                .http.as_ref()?.paths.get(0)?
                                .backend.service.as_ref()?.name.clone())
                        }) {
                            let service_namespace = ingress.metadata.namespace.clone().unwrap_or_default();

                            if let Ok(service_ip) = self.resolve_service_ip(&service_name, &service_namespace).await {
                                let backend_addr = format!("{}:80", service_ip).parse().unwrap();
                                self.load_balancer.remove_backend(&backend_addr);
                                println!("Backend removido para {}: {}", host, backend_addr);
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    async fn resolve_service_ip(&self, service_name: &str, namespace: &str) -> Result<String, Box<dyn Error>> {
        let client = Client::try_default().await?;
        let services: Api<k8s_openapi::api::core::v1::Service> = Api::namespaced(client, namespace);
        if let Some(service) = services.get(service_name).await.ok() {
            if let Some(cluster_ip) = service.spec.and_then(|spec| spec.cluster_ip) {
                return Ok(cluster_ip);
            }
        }
        Err(Box::from("No se pudo resolver el IP del servicio"))
    }
}


