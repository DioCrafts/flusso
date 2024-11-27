// src/observability/handlers.rs
//
//! Handlers para la observabilidad en el API Gateway.
//!
//! Este módulo gestiona métricas, logs y la integración con sistemas de monitoreo como Prometheus.

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

// Modelo para una métrica
#[derive(Serialize, Deserialize, Clone)]
pub struct Metric {
    pub name: String,
    pub value: f64,
    pub labels: Vec<(String, String)>, // Ejemplo: [("route", "/api/v1")]
}

// Modelo para un log
#[derive(Serialize, Deserialize, Clone)]
pub struct LogEntry {
    pub timestamp: String,
    pub client_ip: String,
    pub http_code: u16,
    pub route: String,
    pub backend: String,
    pub latency_ms: u32,
}

// Estado compartido para métricas y logs
pub struct ObservabilityState {
    pub metrics: Mutex<Vec<Metric>>,
    pub logs: Mutex<Vec<LogEntry>>,
    pub alerts: Mutex<Vec<String>>, // Aquí añadimos el campo de alertas
}

impl ObservabilityState {
    pub fn new() -> Self {
        Self {
            metrics: Mutex::new(vec![
                Metric {
                    name: "requests_per_second".to_string(),
                    value: 120.5,
                    labels: vec![("route".to_string(), "/api/v1".to_string())],
                },
                Metric {
                    name: "latency_p95".to_string(),
                    value: 250.0,
                    labels: vec![("route".to_string(), "/api/v1".to_string())],
                },
            ]),
            logs: Mutex::new(vec![
                LogEntry {
                    timestamp: "2024-11-18T12:00:00Z".to_string(),
                    client_ip: "192.168.1.1".to_string(),
                    http_code: 200,
                    route: "/api/v1".to_string(),
                    backend: "backend-1".to_string(),
                    latency_ms: 100,
                },
            ]),
            alerts: Mutex::new(vec![
                "High CPU usage detected".to_string(),
                "Memory consumption exceeded threshold".to_string(),
            ]), // Añadir alertas simuladas
        }
    }
}

// Handlers

/// Obtener métricas
pub async fn get_metrics(data: web::Data<ObservabilityState>) -> impl Responder {
    let metrics = data.metrics.lock().unwrap();
    HttpResponse::Ok().json(&*metrics)
}

/// Obtener logs
pub async fn get_logs(data: web::Data<ObservabilityState>) -> impl Responder {
    let logs = data.logs.lock().unwrap();
    HttpResponse::Ok().json(&*logs)
}

/// Exportar métricas en formato Prometheus
pub async fn export_prometheus_metrics(data: web::Data<ObservabilityState>) -> impl Responder {
    let metrics = data.metrics.lock().unwrap();
    let mut output = String::new();

    for metric in metrics.iter() {
        let labels: String = metric
            .labels
            .iter()
            .map(|(k, v)| format!("{}=\"{}\"", k, v))
            .collect::<Vec<_>>()
            .join(",");
        output.push_str(&format!(
            "{}{{{}}} {}\n",
            metric.name, labels, metric.value
        ));
    }

    HttpResponse::Ok()
        .content_type("text/plain")
        .body(output)
}

/// Agregar un nuevo log manualmente (útil para pruebas)
pub async fn add_log(data: web::Data<ObservabilityState>, log_entry: web::Json<LogEntry>) -> impl Responder {
    let mut logs = data.logs.lock().unwrap();
    logs.push(log_entry.into_inner());
    HttpResponse::Created().finish()
}

/// Obtener alertas
pub async fn get_observability_alerts(data: web::Data<ObservabilityState>) -> impl Responder {
    let alerts = data.alerts.lock().unwrap(); // Acceso a las alertas desde el estado compartido
    HttpResponse::Ok().json(&*alerts)
}
