// src/ingress_controller/event_listener.rs

use kube::{api::{Api, ListParams}, Client};
use kube_runtime::watcher::{watcher, Config, Event as KubeEvent};
use tokio::sync::mpsc;
use crate::ingress_controller::ingress_processor::IngressEvent;
use futures_util::{StreamExt, pin_mut};
use std::error::Error;

#[derive(Clone, Debug)]
pub struct EventListener {
    pub event_channel: mpsc::Sender<IngressEvent>,
}

impl EventListener {
    pub fn new() -> (Self, mpsc::Receiver<IngressEvent>) {
        let (tx, rx) = mpsc::channel(32);
        (Self { event_channel: tx }, rx)
    }

    pub async fn start_listening(&self) -> Result<(), Box<dyn Error>> {
        // Inicializar el cliente de Kubernetes
        let client = Client::try_default().await.expect("Fallo al crear cliente de Kubernetes");
        let ingresses: Api<k8s_openapi::api::networking::v1::Ingress> = Api::all(client);

        // Cargar Ingresses existentes al iniciar
        match ingresses.list(&ListParams::default()).await {
            Ok(ingress_list) => {
                for ingress in ingress_list {
                    if let Some(annotations) = ingress.metadata.annotations {
                        if let Some(class) = annotations.get("kubernetes.io/ingress.class") {
                            if class == "flusso" {
                                if let Some(addr) = ingress.spec.and_then(|spec| {
                                    spec.rules.and_then(|rules| rules.get(0).and_then(|rule| rule.host.clone()))
                                }) {
                                    let addr = format!("{}:80", addr).parse().unwrap();
                                    let ingress_event = IngressEvent::Add(addr);

                                    if let Err(e) = self.event_channel.send(ingress_event).await {
                                        eprintln!("Error al enviar evento inicial al procesador: {:?}", e);
                                    } else {
                                        println!("Evento inicial enviado exitosamente al procesador para host: {}", addr);
                                    }
                                }
                            }
                        }
                    }
                }
            },
            Err(e) => eprintln!("Error al cargar Ingresses existentes: {:?}", e),
        }

        // Crear configuración de watcher para cambios futuros
        let config = Config::default();
        let watcher_stream = watcher(ingresses, config);
        pin_mut!(watcher_stream);

        println!("Watcher de Ingress iniciado, esperando eventos...");

        // Procesar eventos nuevos en el watcher
        while let Some(event) = watcher_stream.next().await {
            match event {
                Ok(KubeEvent::Apply(ingress)) => {
                    println!("Evento Apply capturado para Ingress: {:?}", ingress.metadata.name);
                    if let Some(annotations) = ingress.metadata.annotations {
                        if let Some(class) = annotations.get("kubernetes.io/ingress.class") {
                            if class == "flusso" {
                                if let Some(addr) = ingress.spec.and_then(|spec| {
                                    spec.rules.and_then(|rules| rules.get(0).and_then(|rule| rule.host.clone()))
                                }) {
                                    let addr = format!("{}:80", addr).parse().unwrap();
                                    let ingress_event = IngressEvent::Add(addr);

                                    if let Err(e) = self.event_channel.send(ingress_event).await {
                                        eprintln!("Error al enviar evento al procesador: {:?}", e);
                                    } else {
                                        println!("Evento enviado exitosamente al procesador para host: {}", addr);
                                    }
                                }
                            }
                        }
                    }
                }
                Ok(KubeEvent::Delete(ingress)) => {
                    println!("Evento Delete capturado para Ingress: {:?}", ingress.metadata.name);
                    if let Some(annotations) = ingress.metadata.annotations {
                        if let Some(class) = annotations.get("kubernetes.io/ingress.class") {
                            if class == "flusso" {
                                if let Some(addr) = ingress.spec.and_then(|spec| {
                                    spec.rules.and_then(|rules| rules.get(0).and_then(|rule| rule.host.clone()))
                                }) {
                                    let addr = format!("{}:80", addr).parse().unwrap();
                                    let ingress_event = IngressEvent::Remove(addr);

                                    if let Err(e) = self.event_channel.send(ingress_event).await {
                                        eprintln!("Error al enviar evento de eliminación al procesador: {:?}", e);
                                    } else {
                                        println!("Evento de eliminación enviado exitosamente al procesador para host: {}", addr);
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {
                    println!("Otro tipo de evento recibido, ignorado");
                }
            }
        }

        Ok(())
    }
}

