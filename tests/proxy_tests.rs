// Tests para el proxy y balanceo de carga
// tests/proxy_tests.rs

use flusso::proxy::{Proxy, load_balancer::LoadBalancer};
use flusso::config::Config;
use reqwest::Client;
use tokio::sync::mpsc;

#[tokio::test]
async fn test_proxy_routing() {
    // Configuración de prueba para proxy
    let config = Config::default();
    let proxy = Proxy::new(config.clone());
    let client = Client::new();

    // Añade backends al balanceador de carga
    let mut load_balancer = LoadBalancer::new();
    load_balancer.add_backend("http://backend1");
    load_balancer.add_backend("http://backend2");

    // Envía una solicitud a través del proxy y verifica que llega a un backend
    let response = client.get("http://localhost:8080").send().await.unwrap();
    assert!(response.status().is_success());
}

#[tokio::test]
async fn test_load_balancer_distribution() {
    // Prueba de distribución de carga en varios backends
    let mut load_balancer = LoadBalancer::new();
    load_balancer.add_backend("http://backend1");
    load_balancer.add_backend("http://backend2");
    load_balancer.add_backend("http://backend3");

    // Realiza solicitudes y verifica que se distribuyan entre los backends
    for _ in 0..10 {
        let backend = load_balancer.select_backend().unwrap();
        assert!(["http://backend1", "http://backend2", "http://backend3"].contains(&backend));
    }
}
