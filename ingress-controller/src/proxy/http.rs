//! HTTP Proxy module to forward client requests to backend servers.
//!
//! The `HttpProxy` struct uses a load balancer to select a backend and forwards
//! requests to the chosen backend. It handles HTTP headers, body, and logs request details.

use reqwest::{Client, Response};
use reqwest::header::HeaderMap;
use std::error::Error;
use super::load_balancer::LoadBalancer;
use std::sync::Arc;
use bytes::Bytes;

/// An HTTP proxy that forwards requests to selected backend servers.
pub struct HttpProxy {
    client: Client,
    load_balancer: Arc<LoadBalancer>,
}

impl HttpProxy {
    /// Creates a new `HttpProxy` instance with a load balancer.
    ///
    /// # Parameters
    /// - `load_balancer`: Shared `LoadBalancer` to manage backend selection.
    pub fn new(load_balancer: Arc<LoadBalancer>) -> Self {
        Self {
            client: Client::new(),
            load_balancer,
        }
    }

    /// Forwards a full HTTP request to a backend selected by the load balancer.
    ///
    /// # Parameters
    /// - `path`: The path to forward the request to on the backend.
    /// - `method`: The HTTP method for the request (e.g., GET, POST).
    /// - `headers`: The headers to include in the forwarded request.
    /// - `body`: An optional body for the request.
    ///
    /// # Returns
    /// A `Result` containing the `Response` from the backend or an error.
    pub async fn forward_request(
        &self,
        path: &str,
        method: reqwest::Method,
        headers: HeaderMap,
        body: Option<Bytes>,
    ) -> Result<Response, Box<dyn Error>> {
        println!("Selecting backend for request...");

        if let Some(backend) = self.load_balancer.select_backend() {
            let url = format!("http://{}{}", backend, path);
            println!("Forwarding to URL: {}", url);
            println!("HTTP Method: {:?}", method);
            println!("Headers: {:?}", headers);

            let mut request_builder = self.client.request(method, &url).headers(headers);

            if let Some(b) = body {
                request_builder = request_builder.body(b.clone());
                println!("Request Body: {:?}", b);
            }

            let response = request_builder.send().await;
            match &response {
                Ok(resp) => println!("Backend response: {:?}", resp),
                Err(err) => println!("Error in backend response: {:?}", err),
            }
            response.map_err(|e| e.into())
        } else {
            Err("No available backend".into())
        }
    }
}
