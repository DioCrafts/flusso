//src/rest/handlers.rs
//! REST API Handlers
//!
//! These functions handle incoming HTTP requests and return appropriate responses.

use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use crate::backends::handlers::AppState as BackendsState;
use crate::security::handlers::{AppState as SecurityState, SecurityPolicy};
use crate::observability::handlers::ObservabilityState;

use crate::backends::handlers::Backend;
use kube::Client;
use crate::gateway::crd::gateway::GatewayManager;


/// Handler for `GET /api/backends`
///
/// Delegates to the backend module to list backends.
pub async fn get_backends(data: web::Data<BackendsState>) -> impl Responder {
    crate::backends::handlers::list_backends(data).await
}

/// Handler for `POST /api/backends`
///
/// Adds a new backend service.
pub async fn add_backend(
    app_state: web::Data<BackendsState>,
    backend: web::Json<Backend>,
) -> impl Responder {
    let mut backends = app_state.backends.lock().unwrap();
    let mut new_backend = backend.into_inner();
    new_backend.id = backends.last().map_or(1, |b| b.id + 1);
    new_backend.status = "Unknown".to_string();
    backends.push(new_backend);

    HttpResponse::Created().json(json!({ "message": "Backend added successfully" }))
}

/// Handler for `DELETE /api/backends/{id}`
///
/// Deletes a backend service by ID.
pub async fn delete_backend(
    app_state: web::Data<BackendsState>,
    id: web::Path<u32>,
) -> impl Responder {
    let mut backends = app_state.backends.lock().unwrap();
    let id = id.into_inner();
    let initial_len = backends.len();
    backends.retain(|backend| backend.id != id);

    if backends.len() < initial_len {
        HttpResponse::Ok().json(json!({ "message": "Backend deleted successfully", "id": id }))
    } else {
        HttpResponse::NotFound().json(json!({ "error": "Backend not found", "id": id }))
    }
}

/// Handler for `GET /api/security/policies`
///
/// Retrieves a list of security policies.
pub async fn get_policies(data: web::Data<SecurityState>) -> impl Responder {
    let policies = data.policies.lock().unwrap();
    HttpResponse::Ok().json(&*policies)
}

/// Handler for `POST /api/security/policies`
///
/// Adds a new security policy.
pub async fn add_policy(
    data: web::Data<SecurityState>,
    policy: web::Json<SecurityPolicy>,
) -> impl Responder {
    let mut policies = data.policies.lock().unwrap();
    let mut new_policy = policy.into_inner();
    new_policy.id = policies.last().map_or(1, |p| p.id + 1);
    policies.push(new_policy);

    HttpResponse::Created().json(json!({ "message": "Policy added successfully" }))
}


/// Handler for `GET /api/gateways`
///
/// Returns a list of gateways from the Kubernetes cluster.
pub async fn get_gateways(client: web::Data<Client>) -> impl Responder {
    let manager = GatewayManager::new(client.get_ref().clone());

    match manager.list_gateways("default").await {
        Ok(gateways) => HttpResponse::Ok().json(gateways),
        Err(e) => {
            eprintln!("Error listing Gateways: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to list Gateways.")
        }
    }
}

/// Handler for `POST /api/gateways`
pub async fn add_gateway(
    client: web::Data<Client>,
    gateway: web::Json<crate::gateway::models::Gateway>,
) -> impl Responder {
    let manager = GatewayManager::new(client.get_ref().clone());
    let gateway_spec = crate::gateway::crd::gateway::GatewaySpec {
        spec: crate::gateway::crd::gateway::GatewayInnerSpec {
            hostname: gateway.hostname.clone(),
            tls_enabled: gateway.tls_enabled,
            certificate: gateway.certificate.clone(),
            routes: gateway.routes.iter().cloned().map(Into::into).collect(),
        },
    };

    match manager.create_gateway("default", &gateway_spec).await {
        Ok(_) => HttpResponse::Created().json(json!({ "message": "Gateway created successfully." })),
        Err(e) => {
            eprintln!("Error creating Gateway: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to create Gateway.")
        }
    }
}

/// Handler for `DELETE /api/gateways/{id}`
pub async fn delete_gateway(
    client: web::Data<Client>,
    id: web::Path<String>,
) -> impl Responder {
    let manager = GatewayManager::new(client.get_ref().clone());

    match manager.delete_gateway("default", &id.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json(json!({ "message": "Gateway deleted successfully." })),
        Err(e) => {
            eprintln!("Error deleting Gateway: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to delete Gateway.")
        }
    }
}

/// Handler for `GET /api/metrics`
///
/// Returns a list of metrics (from Observability module).
pub async fn get_metrics(data: web::Data<ObservabilityState>) -> impl Responder {
    let metrics = data.metrics.lock().unwrap();
    HttpResponse::Ok().json(&*metrics)
}

/// Handler for `GET /api/observability/alerts`
///
/// Returns a list of alerts from Observability state.
pub async fn get_observability_alerts(data: web::Data<ObservabilityState>) -> impl Responder {
    let alerts = data.alerts.lock().unwrap(); // Acceso al nuevo campo `alerts`
    HttpResponse::Ok().json(&*alerts)
}


/// Handler for `GET /api/observability/logs`
///
/// Returns a list of logs from Observability state.
pub async fn get_logs(data: web::Data<ObservabilityState>) -> impl Responder {
    let logs = data.logs.lock().unwrap();
    HttpResponse::Ok().json(&*logs)
}


/// Handler for `DELETE /api/security/policies/{id}`
///
/// Deletes a security policy by ID.
pub async fn delete_policy(
    data: web::Data<SecurityState>,
    id: web::Path<u32>,
) -> impl Responder {
    let mut policies = data.policies.lock().unwrap();
    let id = *id;
    let initial_len = policies.len();
    policies.retain(|policy| policy.id != id);

    if policies.len() < initial_len {
        HttpResponse::Ok().json(json!({ "message": "Policy deleted successfully", "id": id }))
    } else {
        HttpResponse::NotFound().json(json!({ "error": "Policy not found", "id": id }))
    }
}

/// Handler for `GET /api/security/protected`
///
/// A protected endpoint for demonstration purposes.
pub async fn protected_endpoint() -> impl Responder {
    HttpResponse::Ok().body("This is a protected endpoint!")
}