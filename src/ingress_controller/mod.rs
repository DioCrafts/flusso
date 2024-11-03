pub mod event_listener;
pub mod ingress_processor;

use crate::proxy::HttpProxy;
use crate::proxy::load_balancer::LoadBalancer;
use event_listener::EventListener;
use ingress_processor::{IngressEvent, IngressProcessor};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct IngressController {
    event_listener: EventListener,
    ingress_processor: IngressProcessor,
    proxy: HttpProxy,
}

impl IngressController {
    pub fn new(load_balancer: Arc<LoadBalancer>) -> Self {
        println!("Inicializando IngressController...");

        // Inicializamos EventListener con el canal y obtenemos tanto el transmisor como el receptor
        let (event_listener, rx) = EventListener::new();
        println!("EventListener inicializado.");

        // Creamos IngressProcessor y le pasamos el receptor para procesar eventos
        let ingress_processor = IngressProcessor::new(load_balancer.clone(), rx);
        println!("IngressProcessor inicializado.");

        let proxy = HttpProxy::new(load_balancer);
        println!("HttpProxy inicializado.");

        println!("IngressController inicializado completamente.");

        Self {
            event_listener,
            ingress_processor,
            proxy,
        }
    }

    /// Inicializa el controlador y comienza a escuchar los eventos de Kubernetes.
    pub async fn start(&self) {
        println!("Comenzando a escuchar eventos con el EventListener...");
        self.event_listener.start_listening().await;
        println!("EventListener escuchando eventos.");
    }

    /// Procesa eventos de Ingress con el IngressProcessor
    pub async fn process_events(&mut self) {
        self.ingress_processor.process_events().await;
    }
}

// Nueva función para iniciar el IngressController con manejo de errores y registros detallados
pub async fn start_ingress_controller(load_balancer: Arc<LoadBalancer>) -> Result<(), Box<dyn std::error::Error>> {
    println!("Iniciando el controlador de ingreso...");
    
    // Utilizar Arc<Mutex<IngressController>> para acceso concurrente sin mover el controlador
    let controller = Arc::new(Mutex::new(IngressController::new(load_balancer)));
    println!("Controlador de ingreso inicializado.");

    // Ejecutamos `start` y `process_events` como tareas concurrentes
    println!("Iniciando el procesamiento de eventos...");

    // Clonamos `controller` para poder pasarlo a ambas tareas de forma concurrente
    let event_listener_task = {
        let controller = Arc::clone(&controller);
        tokio::spawn(async move {
            let controller = controller.lock().await;
            controller.start().await;
        })
    };

    let ingress_processor_task = {
        let controller = Arc::clone(&controller);
        tokio::spawn(async move {
            let mut controller = controller.lock().await;
            controller.process_events().await;
        })
    };

    // Esperamos que ambas tareas finalicen
    let _ = tokio::try_join!(event_listener_task, ingress_processor_task)?;

    println!("Controlador de ingreso iniciado exitosamente.");
    Ok(())
}
