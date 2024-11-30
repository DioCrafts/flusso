// src/handlers/metrics.rs
use actix_web::{get, web, HttpResponse, Result, error::ErrorInternalServerError};
use kube::{
    api::{Api, ListParams},
    Client, ResourceExt,
};
use k8s_openapi::api::core::v1::{Service, Pod};
use prometheus::{Registry, Gauge, Counter};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::collections::BTreeMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize)]
pub struct TrafficPoint {
    time: String,
    value: u32,
}

#[derive(Debug, Serialize)]
pub struct ServiceHealth {
    name: String,
    status: ServiceStatus,
    last_check: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ServiceStatus {
    Healthy,
    Warning,
    Error,
}

#[derive(Clone)]
pub struct MetricsState {
    client: Client,
    registry: Arc<Registry>,
    total_requests: Counter,
    active_services: Gauge,
}

impl MetricsState {
    pub async fn new() -> Result<Self, anyhow::Error> {
        let client = Client::try_default().await?;
        let registry = Registry::new();
        
        let total_requests = Counter::new(
            "gateway_total_requests",
            "Total number of requests processed"
        )?;
        let active_services = Gauge::new(
            "gateway_active_services",
            "Number of active services"
        )?;

        registry.register(Box::new(total_requests.clone()))?;
        registry.register(Box::new(active_services.clone()))?;

        Ok(Self {
            client,
            registry: Arc::new(registry),
            total_requests,
            active_services,
        })
    }
}

#[derive(Serialize)]
pub struct GatewayMetrics {
    total_requests: u64,
    requests_change: f64,
    active_services: u32,
    services_change: i32,
    traffic_data: Vec<TrafficPoint>,
}

#[get("/api/metrics/gateway")]
pub async fn get_gateway_metrics(state: web::Data<MetricsState>) -> Result<HttpResponse> {
    let services: Api<Service> = Api::all(state.client.clone());
    let service_list = services.list(&ListParams::default())
        .await
        .map_err(ErrorInternalServerError)?;
    
    let active_services = service_list
        .iter()
        .filter(|svc| is_service_active(svc))
        .count() as u32;

    state.active_services.set(active_services as f64);

    let traffic_data = get_traffic_metrics(&state.client)
        .await
        .map_err(ErrorInternalServerError)?;

    let metrics = GatewayMetrics {
        total_requests: state.total_requests.get() as u64,
        requests_change: calculate_request_change(&traffic_data),
        active_services,
        services_change: calculate_services_change(&service_list.items),
        traffic_data,
    };

    Ok(HttpResponse::Ok().json(metrics))
}

#[get("/api/services/health")]
pub async fn get_services_health(state: web::Data<MetricsState>) -> Result<HttpResponse> {
    let services: Api<Service> = Api::all(state.client.clone());
    let pods: Api<Pod> = Api::all(state.client.clone());
    
    let service_list = services.list(&ListParams::default())
        .await
        .map_err(ErrorInternalServerError)?;
    let pod_list = pods.list(&ListParams::default())
        .await
        .map_err(ErrorInternalServerError)?;

    let mut service_health = Vec::new();

    for service in service_list.items {
        let service_name = service.metadata.name.as_ref().map(|n| n.clone()).unwrap_or_default();
        let service_pods = get_pods_for_service(&pod_list.items, &service);
        
        let status = calculate_service_health(&service_pods);
        
        service_health.push(ServiceHealth {
            name: service_name,
            status,
            last_check: Utc::now(),
        });
    }

    Ok(HttpResponse::Ok().json(service_health))
}

async fn get_traffic_metrics(_client: &Client) -> Result<Vec<TrafficPoint>, anyhow::Error> {
    Ok(vec![
        TrafficPoint { time: "00:00".to_string(), value: 100 },
        TrafficPoint { time: "01:00".to_string(), value: 150 },
        TrafficPoint { time: "02:00".to_string(), value: 200 },
    ])
}

fn calculate_services_change(services: &[Service]) -> i32 {
    0 // Implementación real pendiente
}

fn is_service_active(service: &Service) -> bool {
    service.spec.is_some() && 
    service.status.is_some() &&
    service.status.as_ref().unwrap().conditions.as_ref()
        .map(|conditions| {
            conditions.iter().any(|condition| 
                condition.type_ == "Ready" && 
                condition.status == "True"
            )
        })
        .unwrap_or(false)
}

fn calculate_request_change(traffic_data: &[TrafficPoint]) -> f64 {
    if traffic_data.len() < 2 {
        return 0.0;
    }
    
    let current = traffic_data.last().unwrap().value as f64;
    let previous = traffic_data[traffic_data.len() - 2].value as f64;
    
    ((current - previous) / previous) * 100.0
}

fn calculate_service_health(pods: &[&Pod]) -> ServiceStatus {
    let total_pods = pods.len();
    if total_pods == 0 {
        return ServiceStatus::Error;
    }

    let ready_pods = pods.iter()
        .filter(|pod| is_pod_ready(pod))
        .count();

    let health_ratio = ready_pods as f32 / total_pods as f32;

    match health_ratio {
        r if r >= 0.9 => ServiceStatus::Healthy,
        r if r >= 0.5 => ServiceStatus::Warning,
        _ => ServiceStatus::Error,
    }
}

fn is_pod_ready(pod: &Pod) -> bool {
    pod.status.as_ref()
        .and_then(|status| status.conditions.as_ref())
        .map(|conditions| {
            conditions.iter().any(|condition| 
                condition.type_ == "Ready" && 
                condition.status == "True"
            )
        })
        .unwrap_or(false)
}

fn get_pods_for_service<'a>(pods: &'a [Pod], service: &Service) -> Vec<&'a Pod> {
    let selector = service.spec.as_ref()
        .and_then(|spec| spec.selector.as_ref());

    match selector {
        Some(selector) => pods.iter()
            .filter(|pod| matches_selector(pod, selector))
            .collect(),
        None => vec![],
    }
}

fn matches_selector(pod: &Pod, selector: &BTreeMap<String, String>) -> bool {
    pod.metadata.labels.as_ref()
        .map(|labels| {
            selector.iter().all(|(k, v)| {
                labels.get(k).map_or(false, |label_value| label_value == v)
            })
        })
        .unwrap_or(false)
}
