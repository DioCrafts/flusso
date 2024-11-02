// Manejo de solicitudes HTTP/S
// src/proxy/http.rs

use reqwest::{Client, Response};
use std::error::Error;
use super::load_balancer::LoadBalancer;

pub struct HttpProxy {
    client: Client,
    load_balancer: LoadBalancer,
}

impl HttpProxy {
    pub fn new(load_balancer: LoadBalancer) -> Self {
        Self {
            client: Client::new(),
            load_balancer,
        }
    }

    /// Reenvía una solicitud al backend seleccionado por el balanceador.
    pub async fn forward_request(
        &self,
        path: &str,
    ) -> Result<Response, Box<dyn Error>> {
        if let Some(backend) = self.load_balancer.select_backend() {
            let url = format!("http://{}{}", backend, path);

            // Enviar la solicitud HTTP GET al backend
            let response = self.client.get(&url).send().await?;
            Ok(response)
        } else {
            Err("No available backend".into())
        }
    }
}
