// src/proxy/http.rs

use reqwest::{Client, Response};
use reqwest::header::HeaderMap;
use std::error::Error;
use super::load_balancer::LoadBalancer;
use std::sync::Arc;
use bytes::Bytes;

pub struct HttpProxy {
    client: Client,
    load_balancer: Arc<LoadBalancer>,
}

impl HttpProxy {
    pub fn new(load_balancer: Arc<LoadBalancer>) -> Self {
        Self {
            client: Client::new(),
            load_balancer,
        }
    }

    /// Reenvía una solicitud HTTP completa al backend seleccionado por el balanceador.
    pub async fn forward_request(
        &self,
        path: &str,
        method: reqwest::Method,
        headers: HeaderMap,
        body: Option<Bytes>,
    ) -> Result<Response, Box<dyn Error>> {

        // Log para indicar que se está intentando seleccionar un backend
        println!("Seleccionando backend para la solicitud...");

        if let Some(backend) = self.load_balancer.select_backend() {
            let url = format!("http://{}{}", backend, path);

            // Añade logs para ver la URL completa y detalles de la solicitud
            println!("Intentando reenviar a URL completa: {}", url);
            println!("Método HTTP: {:?}", method);
            println!("Cabeceras: {:?}", headers);

            let mut request_builder = self.client.request(method, &url).headers(headers);

            // Agrega el cuerpo si existe
            if let Some(b) = body {
                request_builder = request_builder.body(b.clone());
                println!("Cuerpo de la solicitud: {:?}", b);
            }

            // Enviar la solicitud y loggear la respuesta
            let response = request_builder.send().await;
            match response {
                Ok(ref resp) => println!("Respuesta del backend: {:?}", resp),
                Err(ref err) => println!("Error al obtener respuesta del backend: {:?}", err),
            }
            response.map_err(|e| e.into())
        } else {
            Err("No available backend".into())
        }
    }
}

