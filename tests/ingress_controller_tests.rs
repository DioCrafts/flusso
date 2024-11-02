// Tests para el controlador de Ingress
// tests/ingress_controller_tests.rs

use flusso::ingress_controller::{IngressController, IngressEvent};
use tokio::sync::mpsc;

#[tokio::test]
async fn test_ingress_event_processing() {
    // Configuración para prueba
    let (sender, receiver) = mpsc::channel(10);
    let mut controller = IngressController::new(receiver);

    // Simulación de un evento de Ingress
    let ingress_event = IngressEvent::Add {
        name: "test-ingress".to_string(),
        host: "example.com".to_string(),
        backend_url: "http://backend-service".to_string(),
    };
    
    // Envía el evento al controlador y procesa
    sender.send(ingress_event).await.unwrap();
    controller.process_events().await;

    // Verificación: asegúrate de que el controlador procesó el evento
    assert!(controller.routes.contains_key("example.com"));
}
