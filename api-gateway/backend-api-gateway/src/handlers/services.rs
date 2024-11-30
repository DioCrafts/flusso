// src/handlers/services.rs
use actix_web::{get, web, HttpResponse, Result};
use kube::{
    api::{Api, ListParams},
    Client,
};
use k8s_openapi::api::core::v1::Service;
use serde::{Deserialize, Serialize};
use prometheus::Registry;
use std::sync::Arc;

#[derive(Debug, Serialize)]
pub struct ServiceInfo {
    id: String,
    name: String,
    status: ServiceStatus,
    endpoint: String,
    latency: i32,
    uptime: String,
    requests_per_minute: i32,
    error_rate: String,
    last_checked: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ServiceStatus {
    Healthy,
    Warning,
    Error,
}

#[get("/api/services")]
pub async fn get_services(client: web::Data<Client>) -> Result<HttpResponse> {
    let services: Api<Service> = Api::all(client.as_ref());
    let service_list = services.list(&ListParams::default()).await?;

    let mut service_info = Vec::new();
    for service in service_list {
        let name = service.metadata.name.clone().unwrap_or_default();
        let metrics = get_service_metrics(&name).await?;
        let status = calculate_service_status(&service, &metrics);

        service_info.push(ServiceInfo {
            id: service.metadata.uid.clone().unwrap_or_default(),
            name: name.clone(),
            status,
            endpoint: format!("/api/{}", name.to_lowercase()),
            latency: metrics.latency,
            uptime: format!("{:.1}%", metrics.uptime),
            requests_per_minute: metrics.requests_per_minute,
            error_rate: format!("{:.2}%", metrics.error_rate),
            last_checked: chrono::Utc::now(),
        });
    }

    Ok(HttpResponse::Ok().json(service_info))
}

async fn get_service_metrics(service_name: &str) -> Result<ServiceMetrics> {
    // En una implementación real, esto obtendría métricas de Prometheus
    // Por ahora, usamos métricas simuladas
    Ok(ServiceMetrics {
        latency: rand::random::<i32>() % 200 + 20,
        uptime: 99.9,
        requests_per_minute: rand::random::<i32>() % 1000 + 100,
        error_rate: rand::random::<f64>() * 2.0,
    })
}

fn calculate_service_status(
    service: &Service,
    metrics: &ServiceMetrics,
) -> ServiceStatus {
    if metrics.error_rate > 5.0 {
        return ServiceStatus::Error;
    }

    if metrics.latency > 200 || metrics.error_rate > 1.0 {
        return ServiceStatus::Warning;
    }

    ServiceStatus::Healthy
}

struct ServiceMetrics {
    latency: i32,
    uptime: f64,
    requests_per_minute: i32,
    error_rate: f64,
}