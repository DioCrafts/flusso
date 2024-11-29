// src/handlers/proxy.rs
use actix_web::{web, HttpRequest, HttpResponse, http::StatusCode as ActixStatus};
use reqwest::{Client, Method as ReqMethod};
use actix_web::http::header::{HeaderName as ActixHeaderName, HeaderValue as ActixHeaderValue};
use std::str::FromStr;
use crate::config::Route;
use crate::services::load_balancer::LoadBalancer;
use std::sync::Arc;
use futures::StreamExt;
use anyhow::Result;

pub struct ProxyHandler {
    client: Client,
    load_balancer: Arc<LoadBalancer>,
}

impl ProxyHandler {
    pub fn new(load_balancer: Arc<LoadBalancer>) -> Self {
        Self {
            client: Client::new(),
            load_balancer,
        }
    }

    pub async fn handle_request(
        &self,
        route: &Route,
        req: HttpRequest,
        body: web::Bytes,
    ) -> Result<HttpResponse, actix_web::Error> {
        let target = self.load_balancer
            .get_service(&route.target_service)
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

        let target_url = format!("{}{}", target.url, req.uri().path());

        // Convertir el método HTTP
        let method = ReqMethod::from_str(req.method().as_str())
            .map_err(actix_web::error::ErrorInternalServerError)?;

        let mut request_builder = self.client
            .request(method, &target_url)
            .timeout(std::time::Duration::from_secs(route.timeout.unwrap_or(30)));

        // Copiar headers
        for (name, value) in req.headers() {
            if !["host", "connection"].contains(&name.as_str()) {
                request_builder = request_builder.header(name.as_str(), value.as_bytes());
            }
        }

        let response = request_builder
            .body(body)
            .send()
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

        // Convertir el status code a tipo de Actix
        let status_code = ActixStatus::from_u16(response.status().as_u16())
            .map_err(actix_web::error::ErrorInternalServerError)?;

        let mut client_resp = HttpResponse::build(status_code);
        
        // Copiar headers usando tipos de Actix
        for (name, value) in response.headers() {
            if let (Ok(header_name), Ok(header_value)) = (
                ActixHeaderName::from_str(name.as_str()),
                ActixHeaderValue::from_bytes(value.as_bytes())
            ) {
                client_resp.insert_header((header_name, header_value));
            }
        }

        // Streaming del body
        Ok(client_resp.streaming(response.bytes_stream()))
    }

    pub async fn handle_retry(
        &self,
        route: &Route,
        req: HttpRequest,
        body: web::Bytes,
        max_retries: u32,
    ) -> Result<HttpResponse, actix_web::Error> {
        let mut attempts = 0;
        let mut last_error = None;

        while attempts < max_retries {
            match self.handle_request(route, req.clone(), body.clone()).await {
                Ok(response) => return Ok(response),
                Err(error) => {
                    attempts += 1;
                    last_error = Some(error);
                    
                    if attempts < max_retries {
                        tokio::time::sleep(std::time::Duration::from_millis(
                            route.retry.as_ref().map_or(1000, |r| r.backoff_ms)
                        )).await;
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| {
            actix_web::error::ErrorInternalServerError("Max retries exceeded")
        }))
    }
}
