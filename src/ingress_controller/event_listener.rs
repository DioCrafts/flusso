use kube::{api::{Api, ListParams}, Client};
use kube_runtime::watcher::{watcher, Config, Event as KubeEvent};
use tokio::sync::mpsc;
use crate::ingress_controller::ingress_processor::IngressEvent;
use futures_util::{StreamExt, pin_mut};

pub struct EventListener {
    pub event_channel: mpsc::Sender<IngressEvent>,
}

impl EventListener {
    pub fn new() -> (Self, mpsc::Receiver<IngressEvent>) {
        let (tx, rx) = mpsc::channel(32);
        (Self { event_channel: tx }, rx)
    }

    pub async fn start_listening(&self) {
        // Inicializar el cliente de Kubernetes
        let client = Client::try_default().await.expect("Fallo al crear cliente de Kubernetes");
        let ingresses: Api<k8s_openapi::api::networking::v1::Ingress> = Api::all(client);

        // Crear configuración de watcher usando Config
        let config = Config::default();
        let watcher_stream = watcher(ingresses, config);
        pin_mut!(watcher_stream);

        while let Some(event) = watcher_stream.next().await {
            match event {
                Ok(KubeEvent::Apply(ingress)) => {
                    if let Some(annotations) = ingress.metadata.annotations {
                        if let Some(class) = annotations.get("kubernetes.io/ingress.class") {
                            if class == "flusso" {
                                // Convertir el Ingress en un evento y enviarlo
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
                    if let Some(annotations) = ingress.metadata.annotations {
                        if let Some(class) = annotations.get("kubernetes.io/ingress.class") {
                            if class == "flusso" {
                                // Convertir el Ingress en un evento de eliminación
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
                _ => {}
            }
        }
    }
}

