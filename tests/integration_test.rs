// Test de integración general
// tests/integration_test.rs

use flusso::{config::Config, proxy::Proxy, ingress_controller::IngressController};
use reqwest::Client;
use tokio::sync::mpsc;

#[tokio::test]
async fn test_end_to_end_integration() {
    // Configuración y creación de cliente HTTP
    let config = Config::default();
    let proxy = Proxy::new(config.clone());
    let (sender, receiver) = mpsc::channel(10);
    let ingress_controller = IngressController::new(receiver);
    let client = Client::new();

    // Simulación de adición de Ingress y procesamiento de eventos
    let ingress_event = IngressEvent::Add {
        name: "test-ingress".to_string(),
        host: "test.example.com".to_string(),
        backend_url: "http://backend-service".to_string(),
    };
    sender.send(ingress_event).await.unwrap();
    ingress_controller.process_events().await;

    // Prueba de solicitud al proxy
    let response = client.get("http://localhost:8080").send().await.unwrap();
    assert!(response.status().is_success());
}
